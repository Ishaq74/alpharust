//! Interface web : application d'une page (HTML/CSS/JS embarqués) + API JSON + flux SSE.
//!
//! * Aucun JavaScript ni CSS externe : les ressources de `src/gui/assets/` sont compilées dans le binaire.
//! * La génération de projets est un *job* : le thread de travail exécute `scaffold::execute`,
//!   les événements sont conservés (rejouables) et diffusés en SSE à `/api/jobs/:id/events`.
//! * Le serveur n'écoute que sur 127.0.0.1 ; les requêtes `POST` exigent une `Origin`/`Host` locale ;
//!   la destination d'un projet est toujours `sites/<nom>` (jamais un chemin fourni par le client) ;
//!   le lint n'accepte que des chemins sous le dossier courant ou sous `sites/`.
//! * Le client n'insère jamais de texte non échappé dans le DOM (`textContent` / helper `h()`).

use crate::engine::{self, composer};
use crate::parser::Rule;
use crate::profiles::Profile;
use crate::scaffold::astro::{self, PackageManager, Template, TsLevel};
use crate::scaffold::laravel::{Database, Starter};
use crate::scaffold::rust as rust_gen;
use crate::scaffold::{self, Event, Framework, Request};
use axum::{
    extract::{Path as UrlPath, Query, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{
        sse::{Event as SseEvent, KeepAlive, Sse},
        Html, IntoResponse, Response,
    },
    routing::{get, post},
    Json, Router,
};
use futures_util::stream::{self, Stream};
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::watch;

pub const DEFAULT_PORT: u16 = 3000;

const INDEX_HTML: &str = include_str!("assets/index.html");
const APP_CSS: &str = include_str!("assets/app.css");
const APP_JS: &str = include_str!("assets/app.js");
const FAVICON: &str = include_str!("assets/favicon.svg");

/// Nombre maximal de jobs conservés en mémoire.
const MAX_JOBS: usize = 20;

// ---------------------------------------------------------------------------
// Échappement HTML (utilisé pour les rares réponses HTML côté serveur)
// ---------------------------------------------------------------------------

pub fn esc(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            c => out.push(c),
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Jobs de génération
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum JobStatus {
    Running,
    Succeeded,
    Failed,
    Cancelled,
}

impl JobStatus {
    fn as_str(self) -> &'static str {
        match self {
            JobStatus::Running => "running",
            JobStatus::Succeeded => "succeeded",
            JobStatus::Failed => "failed",
            JobStatus::Cancelled => "cancelled",
        }
    }
}

struct Job {
    id: String,
    framework: Framework,
    name: String,
    target: String,
    started_at: u64,
    started: Instant,
    cancel: Arc<AtomicBool>,
    /// Événements sérialisés (JSON), dans l'ordre d'émission.
    events: Mutex<Vec<String>>,
    status: Mutex<JobStatus>,
    elapsed_ms: Mutex<Option<u128>>,
    tick: watch::Sender<usize>,
}

impl Job {
    fn push(&self, ev: &Event) {
        if let Ok(text) = serde_json::to_string(ev) {
            let mut events = self.events.lock().unwrap();
            events.push(text);
            let n = events.len();
            drop(events);
            self.tick.send_replace(n);
        }
    }

    fn status(&self) -> JobStatus {
        *self.status.lock().unwrap()
    }

    fn finish(&self, status: JobStatus) {
        *self.status.lock().unwrap() = status;
        *self.elapsed_ms.lock().unwrap() = Some(self.started.elapsed().as_millis());
        // réveille les flux SSE pour qu'ils constatent la fin
        self.tick.send_modify(|n| *n += 1);
    }

    fn summary(&self) -> Value {
        json!({
            "id": self.id,
            "framework": self.framework.id(),
            "name": self.name,
            "target": self.target,
            "status": self.status().as_str(),
            "started_at": self.started_at,
            "elapsed_ms": *self.elapsed_ms.lock().unwrap(),
            "events": self.events.lock().unwrap().len(),
        })
    }
}

#[derive(Default)]
struct Jobs {
    list: Mutex<Vec<Arc<Job>>>,
}

impl Jobs {
    fn get(&self, id: &str) -> Option<Arc<Job>> {
        self.list
            .lock()
            .unwrap()
            .iter()
            .find(|j| j.id == id)
            .cloned()
    }

    fn running(&self) -> Option<Arc<Job>> {
        self.list
            .lock()
            .unwrap()
            .iter()
            .find(|j| j.status() == JobStatus::Running)
            .cloned()
    }

    fn insert(&self, job: Arc<Job>) {
        let mut list = self.list.lock().unwrap();
        list.push(job);
        while list.len() > MAX_JOBS {
            if let Some(i) = list.iter().position(|j| j.status() != JobStatus::Running) {
                list.remove(i);
            } else {
                break;
            }
        }
    }
}

#[derive(Clone)]
struct AppState {
    port: u16,
    jobs: Arc<Jobs>,
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

// ---------------------------------------------------------------------------
// Utilitaires
// ---------------------------------------------------------------------------

async fn blocking<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> T {
    tokio::task::spawn_blocking(f)
        .await
        .expect("tâche interrompue")
}

fn error(status: StatusCode, msg: impl std::fmt::Display) -> Response {
    (status, Json(json!({ "error": format!("{msg}") }))).into_response()
}

/// Refuse un `POST` dont l'`Origin` ou le `Host` n'est pas l'hôte local de ce serveur.
fn same_origin(headers: &HeaderMap, port: u16) -> bool {
    let allowed = [
        format!("127.0.0.1:{port}"),
        format!("localhost:{port}"),
        format!("[::1]:{port}"),
    ];
    let origin_ok = match headers.get(header::ORIGIN).and_then(|v| v.to_str().ok()) {
        None => true,
        Some(origin) => allowed.iter().any(|a| origin == format!("http://{a}")),
    };
    let host_ok = match headers.get(header::HOST).and_then(|v| v.to_str().ok()) {
        None => true,
        Some(host) => allowed.iter().any(|a| a == host),
    };
    origin_ok && host_ok
}

/// Chemin analysable : sous le dossier courant ou sous `sites/` (après résolution des liens).
fn safe_path(raw: &str) -> Result<PathBuf, String> {
    let raw = if raw.trim().is_empty() {
        "."
    } else {
        raw.trim()
    };
    let p = PathBuf::from(raw);
    let abs = if p.is_absolute() {
        p
    } else {
        std::env::current_dir().map_err(|e| e.to_string())?.join(p)
    };
    let canon = abs
        .canonicalize()
        .map_err(|_| format!("chemin introuvable : {raw}"))?;
    let mut roots = Vec::new();
    if let Ok(cwd) = std::env::current_dir().and_then(|d| d.canonicalize()) {
        roots.push(cwd);
    }
    if let Ok(s) = crate::paths::sites_dir().canonicalize() {
        roots.push(s);
    }
    if roots.iter().any(|r| canon.starts_with(r)) {
        Ok(canon)
    } else {
        Err(format!(
            "chemin refusé : {raw} (seuls le dossier courant et sites/ sont analysables)"
        ))
    }
}

/// Outils + verdict (`Err` = message d'installation) en une seule détection.
fn tools_json(req: &Request) -> (Value, Option<String>) {
    let tools = scaffold::preflight(req);
    let missing = scaffold::preflight_check(&tools)
        .err()
        .map(|e| format!("{e:#}"));
    (json!(tools), missing)
}

// ---------------------------------------------------------------------------
// Statique
// ---------------------------------------------------------------------------

fn asset(content_type: &'static str, body: &'static str) -> Response {
    (
        [
            (header::CONTENT_TYPE, HeaderValue::from_static(content_type)),
            (header::CACHE_CONTROL, HeaderValue::from_static("no-cache")),
            (
                header::HeaderName::from_static("x-content-type-options"),
                HeaderValue::from_static("nosniff"),
            ),
        ],
        body,
    )
        .into_response()
}

async fn index() -> Response {
    let mut res = Html(INDEX_HTML).into_response();
    let h = res.headers_mut();
    h.insert(
        header::CONTENT_SECURITY_POLICY,
        HeaderValue::from_static(
            "default-src 'self'; img-src 'self' data:; style-src 'self'; script-src 'self'; \
             connect-src 'self'; frame-ancestors 'none'; base-uri 'none'; form-action 'self'",
        ),
    );
    h.insert(
        header::HeaderName::from_static("x-frame-options"),
        HeaderValue::from_static("DENY"),
    );
    h.insert(
        header::HeaderName::from_static("referrer-policy"),
        HeaderValue::from_static("no-referrer"),
    );
    res
}

// ---------------------------------------------------------------------------
// API : méta, doctor, plan
// ---------------------------------------------------------------------------

fn framework_schema() -> Value {
    json!([
        {
            "id": "astro", "label": Framework::Astro.label(), "tool": Framework::Astro.official_tool(),
            "description": "Site statique ou hybride : create-astro officiel, intégrations via `astro add`, types vérifiés par `astro check`.",
            "options": {
                "package_manager": PackageManager::ALL.iter().map(|p| p.id()).collect::<Vec<_>>(),
                "template": Template::ALL.iter().map(|t| json!({
                    "id": t.id(),
                    "label": match t { Template::Alpharust => "AlphaRust (minimal + structure complète)", Template::Minimal => "Minimal (officiel)", Template::Basics => "Basics (officiel)", Template::Blog => "Blog (officiel)" },
                })).collect::<Vec<_>>(),
                "typescript": TsLevel::ALL.iter().map(|t| t.id()).collect::<Vec<_>>(),
                "integrations": astro::INTEGRATIONS.iter().map(|(n, d)| json!({"id": n, "description": d})).collect::<Vec<_>>(),
            }
        },
        {
            "id": "laravel", "label": Framework::Laravel.label(), "tool": Framework::Laravel.official_tool(),
            "description": "Application PHP : `composer create-project laravel/laravel`, kit Breeze optionnel, migrations et tests exécutés.",
            "options": {
                "starter": Starter::ALL.iter().map(|s| json!({"id": s.id(), "label": s.label()})).collect::<Vec<_>>(),
                "database": Database::ALL.iter().map(|d| d.id()).collect::<Vec<_>>(),
            }
        },
        {
            "id": "wagtail", "label": Framework::Wagtail.label(), "tool": Framework::Wagtail.official_tool(),
            "description": "CMS Django : venv Python, `pip install wagtail`, `wagtail start`, migrations, check et tests.",
            "options": {}
        },
        {
            "id": "rust", "label": Framework::Rust.label(), "tool": Framework::Rust.official_tool(),
            "description": "Crate Rust : `cargo new`, dépendances via `cargo add`, tests, rustfmt et clippy.",
            "options": {
                "kind": ["bin", "lib"],
                "edition": ["2018", "2021", "2024"],
                "deps": rust_gen::DEPS.iter().map(|(n, f)| json!({"id": n, "features": f})).collect::<Vec<_>>(),
            }
        }
    ])
}

async fn api_meta() -> Json<Value> {
    Json(
        blocking(|| {
            let rules = Rule::load_all().unwrap_or_default();
            let composed = composer::compose(rules.clone());
            let profiles: Vec<Value> = Profile::list()
                .unwrap_or_default()
                .into_iter()
                .map(|name| {
                    let n = Profile::load(&name)
                        .map(|p| p.select(rules.clone()).len())
                        .unwrap_or(0);
                    json!({ "name": name, "rules": n })
                })
                .collect();
            json!({
                "name": "AlphaRust",
                "version": env!("CARGO_PKG_VERSION"),
                "cwd": std::env::current_dir().map(|p| p.display().to_string()).unwrap_or_default(),
                "sites_dir": crate::paths::sites_dir().display().to_string(),
                "rules": composed.len(),
                "checkers": crate::checkers::catalog().len(),
                "profiles": profiles,
                "frameworks": framework_schema(),
            })
        })
        .await,
    )
}

#[derive(Deserialize)]
struct DoctorQuery {
    framework: Option<String>,
}

async fn api_doctor(Query(q): Query<DoctorQuery>) -> Response {
    let frameworks: Vec<Framework> = match q.framework.as_deref().filter(|s| !s.is_empty()) {
        None => Framework::ALL.to_vec(),
        Some(s) => match Framework::parse(s) {
            Some(f) => vec![f],
            None => return error(StatusCode::BAD_REQUEST, format!("framework inconnu : {s}")),
        },
    };
    let out = blocking(move || {
        frameworks
            .into_iter()
            .map(|f| {
                let (tools, missing) = tools_json(&Request::new(f, "doctor"));
                json!({ "framework": f.id(), "label": f.label(), "ready": missing.is_none(), "missing": missing, "tools": tools })
            })
            .collect::<Vec<_>>()
    })
    .await;
    Json(json!(out)).into_response()
}

/// La destination est imposée : `sites/<nom>`.
fn sanitize_request(mut req: Request) -> Request {
    req.path = None;
    req
}

async fn api_plan(
    State(st): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<Request>,
) -> Response {
    if !same_origin(&headers, st.port) {
        return error(StatusCode::FORBIDDEN, "origine refusée");
    }
    let req = sanitize_request(req);
    if let Err(e) = req.validate() {
        return error(StatusCode::UNPROCESSABLE_ENTITY, format!("{e:#}"));
    }
    blocking(move || match scaffold::plan(&req) {
        Ok(plan) => {
            let (tools, missing) = tools_json(&req);
            let ready = missing.is_none();
            Json(json!({
                "target": plan.target.display().to_string(),
                "steps": plan.infos(),
                "preview": plan.preview(),
                "tools": tools,
                "ready": ready,
                "missing": missing,
                "exists": plan.target.exists(),
            }))
            .into_response()
        }
        Err(e) => error(StatusCode::UNPROCESSABLE_ENTITY, format!("{e:#}")),
    })
    .await
}

// ---------------------------------------------------------------------------
// API : jobs
// ---------------------------------------------------------------------------

async fn api_job_create(
    State(st): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<Request>,
) -> Response {
    if !same_origin(&headers, st.port) {
        return error(StatusCode::FORBIDDEN, "origine refusée");
    }
    let req = sanitize_request(req);
    if let Err(e) = req.validate() {
        return error(StatusCode::UNPROCESSABLE_ENTITY, format!("{e:#}"));
    }
    if let Some(j) = st.jobs.running() {
        return error(
            StatusCode::CONFLICT,
            format!(
                "une génération est déjà en cours ({} « {} »)",
                j.framework.label(),
                j.name
            ),
        );
    }
    let id = format!("{:x}{:04x}", now_secs(), st.jobs.list.lock().unwrap().len());
    let (tick, _) = watch::channel(0usize);
    let job = Arc::new(Job {
        id: id.clone(),
        framework: req.framework(),
        name: req.name.clone(),
        target: req.target().display().to_string(),
        started_at: now_secs(),
        started: Instant::now(),
        cancel: Arc::new(AtomicBool::new(false)),
        events: Mutex::new(Vec::new()),
        status: Mutex::new(JobStatus::Running),
        elapsed_ms: Mutex::new(None),
        tick,
    });
    st.jobs.insert(job.clone());
    let worker = job.clone();
    std::thread::spawn(move || {
        let result = scaffold::execute(&req, &mut |ev| worker.push(&ev), &worker.cancel);
        let status = match result {
            Ok(_) => JobStatus::Succeeded,
            Err(_) if worker.cancel.load(Ordering::Relaxed) => JobStatus::Cancelled,
            Err(_) => JobStatus::Failed,
        };
        worker.finish(status);
    });
    (StatusCode::ACCEPTED, Json(job.summary())).into_response()
}

async fn api_jobs(State(st): State<AppState>) -> Json<Value> {
    let list = st.jobs.list.lock().unwrap();
    Json(json!(list
        .iter()
        .rev()
        .map(|j| j.summary())
        .collect::<Vec<_>>()))
}

async fn api_job(State(st): State<AppState>, UrlPath(id): UrlPath<String>) -> Response {
    match st.jobs.get(&id) {
        Some(j) => Json(j.summary()).into_response(),
        None => error(StatusCode::NOT_FOUND, "job introuvable"),
    }
}

async fn api_job_cancel(
    State(st): State<AppState>,
    headers: HeaderMap,
    UrlPath(id): UrlPath<String>,
) -> Response {
    if !same_origin(&headers, st.port) {
        return error(StatusCode::FORBIDDEN, "origine refusée");
    }
    match st.jobs.get(&id) {
        Some(j) => {
            j.cancel.store(true, Ordering::Relaxed);
            Json(j.summary()).into_response()
        }
        None => error(StatusCode::NOT_FOUND, "job introuvable"),
    }
}

#[derive(Deserialize)]
struct EventsQuery {
    /// Reprise : index du premier événement à envoyer.
    from: Option<usize>,
}

/// Flux SSE : rejoue les événements déjà émis puis diffuse les suivants ; se termine avec le job.
async fn api_job_events(
    State(st): State<AppState>,
    UrlPath(id): UrlPath<String>,
    Query(q): Query<EventsQuery>,
    headers: HeaderMap,
) -> Response {
    let Some(job) = st.jobs.get(&id) else {
        return error(StatusCode::NOT_FOUND, "job introuvable");
    };
    let from = q
        .from
        .or_else(|| {
            headers
                .get("last-event-id")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<usize>().ok())
                .map(|i| i + 1)
        })
        .unwrap_or(0);
    let rx = job.tick.subscribe();
    let s: std::pin::Pin<Box<dyn Stream<Item = Result<SseEvent, Infallible>> + Send>> = Box::pin(
        stream::unfold(
            (job, rx, from, false),
            |(job, mut rx, mut idx, closed)| async move {
                if closed {
                    return None;
                }
                loop {
                    let next = job.events.lock().unwrap().get(idx).cloned();
                    if let Some(text) = next {
                        let ev = SseEvent::default().id(idx.to_string()).data(text);
                        idx += 1;
                        return Some((Ok(ev), (job, rx, idx, false)));
                    }
                    if job.status() != JobStatus::Running {
                        // tous les événements sont partis : message de clôture
                        let end = SseEvent::default().event("end").data(
                        json!({ "status": job.status().as_str(), "elapsed_ms": *job.elapsed_ms.lock().unwrap() })
                            .to_string(),
                    );
                        return Some((Ok(end), (job, rx, idx, true)));
                    }
                    if rx.changed().await.is_err() {
                        return None;
                    }
                }
            },
        ),
    );
    Sse::new(s).keep_alive(KeepAlive::default()).into_response()
}

// ---------------------------------------------------------------------------
// API : projets, lint, règles
// ---------------------------------------------------------------------------

async fn api_projects() -> Json<Value> {
    Json(
        blocking(|| {
            let dir = crate::paths::sites_dir();
            let mut out = Vec::new();
            if let Ok(rd) = std::fs::read_dir(&dir) {
                for e in rd.flatten() {
                    let p = e.path();
                    if !p.is_dir() {
                        continue;
                    }
                    let name = e.file_name().to_string_lossy().into_owned();
                    let manifest = std::fs::read_to_string(p.join(scaffold::MANIFEST))
                        .ok()
                        .and_then(|t| serde_json::from_str::<Value>(&t).ok());
                    out.push(json!({
                        "name": name,
                        "path": p.display().to_string(),
                        "generated": manifest.is_some(),
                        "framework": manifest.as_ref().and_then(|m| m.get("framework")).cloned(),
                        "created": manifest.as_ref().and_then(|m| m.get("created_at")).cloned(),
                        "official_tool": manifest.as_ref().and_then(|m| m.get("official_tool")).cloned(),
                    }));
                }
            }
            out.sort_by(|a, b| a["name"].as_str().cmp(&b["name"].as_str()));
            json!({ "dir": dir.display().to_string(), "projects": out })
        })
        .await,
    )
}

#[derive(Deserialize)]
struct LintQuery {
    path: Option<String>,
    profile: Option<String>,
}

fn run_lint(q: &LintQuery) -> Result<(PathBuf, engine::LintReport, u128), (StatusCode, String)> {
    let path =
        safe_path(q.path.as_deref().unwrap_or(".")).map_err(|e| (StatusCode::BAD_REQUEST, e))?;
    let profile = q.profile.clone().filter(|p| !p.is_empty());
    let t = Instant::now();
    engine::lint(&path, profile.as_deref())
        .map(|r| (path, r, t.elapsed().as_millis()))
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("{e:#}")))
}

async fn api_lint(Query(q): Query<LintQuery>) -> Response {
    blocking(move || match run_lint(&q) {
        Ok((path, r, ms)) => Json(json!({
            "path": path.display().to_string(),
            "profile": r.profile,
            "files_scanned": r.files_scanned,
            "rules_used": r.rules_used,
            "elapsed_ms": ms,
            "counts": {
                "error": r.count("error"),
                "warning": r.count("warning"),
                "info": r.count("info"),
            },
            "violations": r.violations,
        }))
        .into_response(),
        Err((status, msg)) => error(status, msg),
    })
    .await
}

async fn api_sarif(Query(q): Query<LintQuery>) -> Response {
    blocking(move || match run_lint(&q) {
        Ok((_, r, _)) => (
            [
                (header::CONTENT_TYPE, "application/sarif+json"),
                (
                    header::CONTENT_DISPOSITION,
                    "attachment; filename=\"alpharust.sarif\"",
                ),
            ],
            crate::sarif::to_sarif(&r.violations),
        )
            .into_response(),
        Err((status, msg)) => error(status, msg),
    })
    .await
}

fn rule_summary(r: &Rule) -> Value {
    json!({
        "id": r.id,
        "name": r.name,
        "category": r.id.split('/').next().unwrap_or(""),
        "severity": r.severity,
        "apply": r.apply,
        "kind": r.kind(),
        "message": r.display_message(),
    })
}

async fn api_rules() -> Json<Value> {
    Json(
        blocking(|| {
            let rules = composer::compose(Rule::load_all().unwrap_or_default());
            json!(rules.iter().map(rule_summary).collect::<Vec<_>>())
        })
        .await,
    )
}

async fn api_rule(Query(q): Query<HashMap<String, String>>) -> Response {
    let id = q.get("id").cloned().unwrap_or_default();
    blocking(move || {
        let rules = Rule::load_all().unwrap_or_default();
        match rules.iter().find(|r| r.id == id) {
            Some(r) => {
                let mut v = rule_summary(r);
                v["pattern"] = json!(r.pattern);
                v["checker"] = json!(r.checker);
                v["args"] = json!(r.args);
                v["description"] = json!(r.description);
                v["examples"] = json!({ "bad": r.examples.bad, "good": r.examples.good });
                Json(v).into_response()
            }
            None => error(StatusCode::NOT_FOUND, format!("règle introuvable : {id}")),
        }
    })
    .await
}

async fn api_checkers() -> Json<Value> {
    Json(json!(crate::checkers::catalog()
        .iter()
        .map(|(n, d)| json!({ "name": n, "description": d }))
        .collect::<Vec<_>>()))
}

// ---------------------------------------------------------------------------
// Routeur
// ---------------------------------------------------------------------------

pub fn router(port: u16) -> Router {
    let state = AppState {
        port,
        jobs: Arc::new(Jobs::default()),
    };
    Router::new()
        .route("/", get(index))
        .route(
            "/assets/app.css",
            get(|| async { asset("text/css; charset=utf-8", APP_CSS) }),
        )
        .route(
            "/assets/app.js",
            get(|| async { asset("text/javascript; charset=utf-8", APP_JS) }),
        )
        .route(
            "/favicon.svg",
            get(|| async { asset("image/svg+xml", FAVICON) }),
        )
        .route("/api/meta", get(api_meta))
        .route("/api/doctor", get(api_doctor))
        .route("/api/plan", post(api_plan))
        .route("/api/jobs", get(api_jobs).post(api_job_create))
        .route("/api/jobs/:id", get(api_job))
        .route("/api/jobs/:id/cancel", post(api_job_cancel))
        .route("/api/jobs/:id/events", get(api_job_events))
        .route("/api/projects", get(api_projects))
        .route("/api/lint", get(api_lint))
        .route("/api/sarif", get(api_sarif))
        .route("/api/rules", get(api_rules))
        .route("/api/rule", get(api_rule))
        .route("/api/checkers", get(api_checkers))
        .route("/healthz", get(|| async { "ok" }))
        .with_state(state)
}

pub async fn run(port: u16) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| anyhow::anyhow!("impossible d'écouter sur {} : {}", addr, e))?;
    println!("AlphaRust GUI running at http://{}", addr);
    axum::serve(listener, router(port)).await?;
    Ok(())
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn esc_escapes_all_special_characters() {
        assert_eq!(
            esc("<a href=\"x\">&'"),
            "&lt;a href=&quot;x&quot;&gt;&amp;&#39;"
        );
    }

    #[test]
    fn origin_and_host_checks() {
        let mut h = HeaderMap::new();
        assert!(same_origin(&h, 3000));
        h.insert(header::ORIGIN, "http://localhost:3000".parse().unwrap());
        assert!(same_origin(&h, 3000));
        h.insert(header::HOST, "evil.example:3000".parse().unwrap());
        assert!(!same_origin(&h, 3000), "DNS rebinding");
        h.insert(header::HOST, "127.0.0.1:3000".parse().unwrap());
        // alpharust-ignore: no-hardcoded-urls
        h.insert(header::ORIGIN, "http://evil.example".parse().unwrap());
        assert!(!same_origin(&h, 3000));
    }

    #[test]
    fn safe_path_rejects_outside_roots() {
        assert!(safe_path("").is_ok());
        assert!(safe_path(".").is_ok());
        assert!(safe_path("/etc").is_err());
        assert!(safe_path("../../../../etc").is_err());
        assert!(safe_path("/definitely/not/here").is_err());
    }

    #[test]
    fn jobs_keep_a_bounded_history() {
        let jobs = Jobs::default();
        for i in 0..(MAX_JOBS + 5) {
            let (tick, _) = watch::channel(0);
            let job = Arc::new(Job {
                id: i.to_string(),
                framework: Framework::Rust,
                name: "x".into(),
                target: "t".into(),
                started_at: 0,
                started: Instant::now(),
                cancel: Arc::new(AtomicBool::new(false)),
                events: Mutex::new(vec![]),
                status: Mutex::new(JobStatus::Succeeded),
                elapsed_ms: Mutex::new(None),
                tick,
            });
            jobs.insert(job);
        }
        assert_eq!(jobs.list.lock().unwrap().len(), MAX_JOBS);
        assert!(jobs.get("0").is_none() && jobs.get("24").is_some());
    }
}

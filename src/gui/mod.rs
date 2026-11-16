//! GUI web locale (axum) : tableau de bord, règles, génération, lint, API JSON/SARIF.
//!
//! Le serveur n'écoute que sur 127.0.0.1. Les requêtes POST dont l'en-tête `Origin`
//! ne correspond pas à l'hôte local sont refusées (protection CSRF).

use crate::engine;
use crate::generators::{self, Kind};
use crate::parser::Rule;
use crate::profiles::Profile;
use axum::{
    extract::{Form, Query},
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::Path;

pub const DEFAULT_PORT: u16 = 3000;

/// Échappe le HTML (tout texte dynamique passe par ici).
pub fn esc(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

const CSS: &str = r#"
:root{--bg:#0d1117;--panel:#161b22;--line:#30363d;--fg:#e6edf3;--muted:#8b949e;--accent:#58a6ff;--err:#f85149;--warn:#d29922;--info:#3fb950}
*{box-sizing:border-box}
body{margin:0;font-family:system-ui,-apple-system,Segoe UI,sans-serif;background:var(--bg);color:var(--fg);line-height:1.5}
header.top{background:var(--panel);border-bottom:1px solid var(--line);padding:.8rem 2rem;display:flex;gap:2rem;align-items:center}
header.top .brand{font-weight:700;font-size:1.2rem;color:var(--fg);text-decoration:none}
header.top nav a{color:var(--muted);text-decoration:none;margin-right:1.2rem;padding-bottom:.2rem}
header.top nav a.active,header.top nav a:hover{color:var(--fg);border-bottom:2px solid var(--accent)}
main{max-width:1100px;margin:0 auto;padding:1.5rem 2rem 3rem}
h1{margin:.2rem 0 1rem;font-size:1.7rem}
a{color:var(--accent)}
.cards{display:grid;grid-template-columns:repeat(auto-fit,minmax(200px,1fr));gap:1rem;margin:1rem 0 2rem}
.card{background:var(--panel);border:1px solid var(--line);border-radius:8px;padding:1rem 1.2rem}
.card .n{font-size:2rem;font-weight:700}.card .l{color:var(--muted)}
table{width:100%;border-collapse:collapse;background:var(--panel);border:1px solid var(--line);border-radius:8px;overflow:hidden}
th,td{text-align:left;padding:.5rem .8rem;border-bottom:1px solid var(--line);vertical-align:top}
th{color:var(--muted);font-weight:600;font-size:.85rem;text-transform:uppercase}
tr:last-child td{border-bottom:0}
code,pre{font-family:ui-monospace,SFMono-Regular,Menlo,monospace;font-size:.88rem}
pre{background:var(--panel);border:1px solid var(--line);border-radius:8px;padding:1rem;overflow:auto;white-space:pre-wrap}
form.inline{display:flex;gap:.6rem;flex-wrap:wrap;align-items:end;background:var(--panel);border:1px solid var(--line);border-radius:8px;padding:1rem;margin:1rem 0}
label{display:flex;flex-direction:column;font-size:.8rem;color:var(--muted);gap:.25rem}
input,select{background:var(--bg);color:var(--fg);border:1px solid var(--line);border-radius:6px;padding:.5rem .7rem;font-size:.95rem;min-width:12rem}
button{background:#238636;color:#fff;border:0;border-radius:6px;padding:.55rem 1.1rem;font-size:.95rem;cursor:pointer}
button:hover{background:#2ea043}
.badge{display:inline-block;padding:.05rem .55rem;border-radius:999px;font-size:.78rem;font-weight:600;border:1px solid}
.badge.error{color:var(--err);border-color:var(--err)}.badge.warning{color:var(--warn);border-color:var(--warn)}
.badge.info{color:var(--info);border-color:var(--info)}.badge.kind{color:var(--muted);border-color:var(--line)}
.ok{color:var(--info);font-weight:600}.fail{color:var(--err);font-weight:600}
.muted{color:var(--muted)}
.summary{display:flex;gap:1.5rem;margin:1rem 0;font-size:1.05rem}
"#;

/// Gabarit commun à toutes les pages.
pub fn page(title: &str, active: &str, body: &str) -> String {
    let nav = [
        ("/", "Tableau de bord"),
        ("/rules", "Règles"),
        ("/generate", "Générer"),
        ("/lint", "Lint"),
        ("/checkers", "Checkers"),
    ]
    .iter()
    .map(|(href, label)| {
        let class = if *href == active {
            " class=\"active\""
        } else {
            ""
        };
        format!("<a href=\"{}\"{}>{}</a>", href, class, label)
    })
    .collect::<Vec<_>>()
    .join("");
    format!(
        "<!DOCTYPE html>\n<html lang=\"fr\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><title>{} — AlphaRust</title><style>{}</style></head><body><header class=\"top\"><a class=\"brand\" href=\"/\">⚙ AlphaRust</a><nav>{}</nav></header><main>{}</main></body></html>",
        esc(title),
        CSS,
        nav,
        body
    )
}

fn badge(severity: &str) -> String {
    format!("<span class=\"badge {0}\">{0}</span>", esc(severity))
}

fn sites() -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(crate::paths::sites_dir())
        .map(|d| {
            d.filter_map(|e| e.ok())
                .filter(|e| e.path().is_dir())
                .filter_map(|e| e.file_name().into_string().ok())
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    names
}

// ---------------------------------------------------------------------------
// Pages
// ---------------------------------------------------------------------------

pub fn dashboard_html() -> String {
    let rules = Rule::load_all().unwrap_or_default();
    let count = |s: &str| rules.iter().filter(|r| r.severity == s).count();
    let profiles = Profile::list().unwrap_or_default();
    let sites = sites();
    let mut body = format!(
        "<h1>AlphaRust</h1><p class=\"muted\">Règles de qualité, générateurs de projets et lint — v{}</p>",
        env!("CARGO_PKG_VERSION")
    );
    body.push_str(&format!(
        "<div class=\"cards\"><div class=\"card\"><div class=\"n\">{}</div><div class=\"l\">règles</div></div>\
         <div class=\"card\"><div class=\"n\">{}</div><div class=\"l\">erreurs · {} warnings · {} infos</div></div>\
         <div class=\"card\"><div class=\"n\">{}</div><div class=\"l\">profils</div></div>\
         <div class=\"card\"><div class=\"n\">{}</div><div class=\"l\">checkers sémantiques</div></div>\
         <div class=\"card\"><div class=\"n\">{}</div><div class=\"l\">projets dans sites/</div></div></div>",
        rules.len(),
        count("error"),
        count("warning"),
        count("info"),
        profiles.len(),
        crate::checkers::catalog().len(),
        sites.len()
    ));
    body.push_str("<h2>Profils</h2><table><tr><th>Profil</th><th>Règles</th><th>Lint</th></tr>");
    for name in &profiles {
        if let Ok(p) = Profile::load(name) {
            body.push_str(&format!(
                "<tr><td><strong>{0}</strong></td><td>{1}</td><td><a href=\"/lint?path=tests/fixtures/violations&amp;profile={0}\">lint des fixtures</a></td></tr>",
                esc(name),
                p.select(rules.clone()).len()
            ));
        }
    }
    body.push_str("</table><h2>Projets générés</h2>");
    if sites.is_empty() {
        body.push_str("<p class=\"muted\">Aucun projet pour le moment — <a href=\"/generate\">en générer un</a>.</p>");
    } else {
        body.push_str("<ul>");
        for s in &sites {
            body.push_str(&format!("<li><code>sites/{}</code></li>", esc(s)));
        }
        body.push_str("</ul>");
    }
    page("Tableau de bord", "/", &body)
}

#[derive(Deserialize, Default)]
pub struct RulesQuery {
    q: Option<String>,
    severity: Option<String>,
}

pub fn rules_html(query: &RulesQuery) -> String {
    let all = Rule::load_all().unwrap_or_default();
    let q = query.q.clone().unwrap_or_default().to_lowercase();
    let sev = query.severity.clone().unwrap_or_default();
    let rules: Vec<&Rule> = all
        .iter()
        .filter(|r| sev.is_empty() || r.severity == sev)
        .filter(|r| {
            q.is_empty()
                || r.id.to_lowercase().contains(&q)
                || r.name.to_lowercase().contains(&q)
                || r.apply.to_lowercase().contains(&q)
        })
        .collect();
    let opt = |v: &str, label: &str| {
        format!(
            "<option value=\"{0}\"{1}>{2}</option>",
            v,
            if sev == v { " selected" } else { "" },
            label
        )
    };
    let mut body = format!(
        "<h1>Règles</h1><form class=\"inline\" method=\"get\" action=\"/rules\"><label>Recherche<input name=\"q\" value=\"{}\" placeholder=\"stripe, eval, html…\"></label><label>Sévérité<select name=\"severity\">{}{}{}{}</select></label><button type=\"submit\">Filtrer</button></form><p class=\"muted\">{} règle(s) sur {} — <a href=\"/api/rules\">JSON</a></p>",
        esc(&query.q.clone().unwrap_or_default()),
        opt("", "toutes"),
        opt("error", "error"),
        opt("warning", "warning"),
        opt("info", "info"),
        rules.len(),
        all.len()
    );
    body.push_str("<table><tr><th>Règle</th><th>Sévérité</th><th>Type</th><th>Apply</th></tr>");
    for r in rules {
        body.push_str(&format!(
            "<tr><td><a href=\"/rule?id={0}\"><code>{0}</code></a><br><span class=\"muted\">{1}</span></td><td>{2}</td><td><span class=\"badge kind\">{3}</span></td><td><code>{4}</code></td></tr>",
            esc(&r.id),
            esc(&r.display_message()),
            badge(&r.severity),
            r.kind(),
            esc(&r.apply)
        ));
    }
    body.push_str("</table>");
    page("Règles", "/rules", &body)
}

pub fn rule_html(id: &str) -> Option<String> {
    let rules = Rule::load_all().ok()?;
    let r = rules.iter().find(|r| r.id == id)?;
    let mut body = format!(
        "<h1><code>{}</code></h1><p>{} <span class=\"badge kind\">{}</span> — apply : <code>{}</code></p>",
        esc(&r.id),
        badge(&r.severity),
        r.kind(),
        esc(&r.apply)
    );
    if let Some(p) = &r.pattern {
        body.push_str(&format!("<h3>Pattern</h3><pre>{}</pre>", esc(p)));
    }
    if let Some(c) = &r.checker {
        body.push_str(&format!("<h3>Checker</h3><pre>{}</pre>", esc(c)));
    }
    body.push_str(&format!(
        "<h3>Description</h3><pre>{}</pre><p><a href=\"/rules\">← Règles</a></p>",
        esc(&r.description)
    ));
    Some(page(&r.id, "/rules", &body))
}

pub fn checkers_html() -> String {
    let mut body = String::from("<h1>Checkers sémantiques</h1><p class=\"muted\">Analyses au-delà de la regex ligne à ligne : balises HTML, titres Markdown, lexer Rust, contraste CSS, manifestes, présence de fichiers…</p><table><tr><th>Checker</th><th>Description</th><th>Règles</th></tr>");
    let rules = Rule::load_all().unwrap_or_default();
    for (name, desc) in crate::checkers::catalog() {
        let used: Vec<String> = rules
            .iter()
            .filter(|r| r.checker.as_deref() == Some(name))
            .map(|r| format!("<code>{}</code>", esc(&r.id)))
            .collect();
        body.push_str(&format!(
            "<tr><td><code>{}</code></td><td>{}</td><td>{}</td></tr>",
            name,
            esc(desc),
            used.join("<br>")
        ));
    }
    body.push_str("</table>");
    page("Checkers", "/checkers", &body)
}

fn generate_form_html(message: Option<&str>) -> String {
    let mut body = String::from("<h1>Générer un projet</h1>");
    if let Some(m) = message {
        body.push_str(m);
    }
    body.push_str(
        "<form class=\"inline\" method=\"post\" action=\"/generate\"><label>Type<select name=\"kind\"><option value=\"wagtail\">Wagtail (Django CMS)</option><option value=\"astro\">Astro</option><option value=\"laravel\">Laravel</option><option value=\"init\">Rust (init)</option></select></label><label>Nom du projet<input type=\"text\" name=\"name\" placeholder=\"mon-site\" required pattern=\"[A-Za-z0-9][A-Za-z0-9._\\-]*\"></label><button type=\"submit\">Générer dans sites/</button></form><h2>Projets existants</h2>",
    );
    let sites = sites();
    if sites.is_empty() {
        body.push_str("<p class=\"muted\">Aucun projet dans <code>sites/</code>.</p>");
    } else {
        body.push_str("<ul>");
        for s in sites {
            body.push_str(&format!("<li><code>sites/{}</code></li>", esc(&s)));
        }
        body.push_str("</ul>");
    }
    page("Générer", "/generate", &body)
}

#[derive(Deserialize)]
pub struct GenerateForm {
    kind: String,
    name: String,
}

/// Exécute la génération et renvoie (statut, page HTML).
pub fn generate_result(form: &GenerateForm) -> (StatusCode, String) {
    let Some(kind) = Kind::parse(&form.kind) else {
        let msg = format!(
            "<div class=\"card\"><span class=\"fail\">✗ Type inconnu : {}</span></div>",
            esc(&form.kind)
        );
        return (StatusCode::BAD_REQUEST, generate_form_html(Some(&msg)));
    };
    let target = generators::default_target(&form.name);
    match generators::generate(kind, &target, &form.name, false) {
        Ok(g) => {
            let files: String = g.files.iter().map(|f| format!("{}\n", esc(f))).collect();
            let msg = format!(
                "<div class=\"card\"><span class=\"ok\">✓ Projet {} « {} » créé dans sites/{}</span> — {} fichiers</div><pre>{}</pre>",
                kind.label(),
                esc(&form.name),
                esc(&form.name),
                g.files.len(),
                files
            );
            (StatusCode::OK, generate_form_html(Some(&msg)))
        }
        Err(e) => {
            let msg = format!(
                "<div class=\"card\"><span class=\"fail\">✗ {}</span></div>",
                esc(&format!("{:#}", e))
            );
            (StatusCode::BAD_REQUEST, generate_form_html(Some(&msg)))
        }
    }
}

#[derive(Deserialize, Default, Clone)]
pub struct LintQuery {
    path: Option<String>,
    profile: Option<String>,
}

pub fn lint_html(query: &LintQuery) -> String {
    let profiles = Profile::list().unwrap_or_default();
    let path = query.path.clone().unwrap_or_default();
    let profile = query.profile.clone().unwrap_or_default();
    let mut options = format!(
        "<option value=\"\"{}>toutes les règles</option>",
        if profile.is_empty() { " selected" } else { "" }
    );
    for p in &profiles {
        options.push_str(&format!(
            "<option value=\"{0}\"{1}>{0}</option>",
            esc(p),
            if *p == profile { " selected" } else { "" }
        ));
    }
    let mut body = format!(
        "<h1>Lint</h1><form class=\"inline\" method=\"get\" action=\"/lint\"><label>Chemin<input type=\"text\" name=\"path\" value=\"{}\" placeholder=\".\"></label><label>Profil<select name=\"profile\">{}</select></label><button type=\"submit\">Lancer le lint</button></form>",
        esc(if path.is_empty() { "." } else { &path }),
        options
    );
    if !path.is_empty() {
        let profile_opt = (!profile.is_empty()).then_some(profile.as_str());
        match engine::lint(Path::new(&path), profile_opt) {
            Ok(report) => {
                let (e, w, i) = (
                    report.count("error"),
                    report.count("warning"),
                    report.count("info"),
                );
                let q = format!("path={}&amp;profile={}", esc(&path), esc(&profile));
                body.push_str(&format!(
                    "<div class=\"summary\"><span class=\"{}\">{} violation(s)</span><span>{} error</span><span>{} warning</span><span>{} info</span><span class=\"muted\">{} fichiers · {} règles{}</span></div><p><a href=\"/api/lint?{q}\">JSON</a> · <a href=\"/api/sarif?{q}\">SARIF</a></p>",
                    if report.violations.is_empty() { "ok" } else { "fail" },
                    report.violations.len(),
                    e,
                    w,
                    i,
                    report.files_scanned,
                    report.rules_used,
                    report.profile.as_ref().map(|p| format!(" · profil {}", esc(p))).unwrap_or_default(),
                    q = q
                ));
                if report.violations.is_empty() {
                    body.push_str("<p class=\"ok\">✓ Aucune violation</p>");
                } else {
                    body.push_str("<table><tr><th>Sévérité</th><th>Fichier</th><th>Règle</th><th>Message</th></tr>");
                    for v in &report.violations {
                        body.push_str(&format!(
                            "<tr><td>{}</td><td><code>{}:{}</code></td><td><code>{}</code></td><td>{}</td></tr>",
                            badge(&v.severity),
                            esc(&v.file),
                            v.line,
                            esc(&v.rule_id),
                            esc(&v.message)
                        ));
                    }
                    body.push_str("</table>");
                }
            }
            Err(e) => body.push_str(&format!(
                "<div class=\"card\"><span class=\"fail\">✗ {}</span></div>",
                esc(&format!("{:#}", e))
            )),
        }
    }
    page("Lint", "/lint", &body)
}

// ---------------------------------------------------------------------------
// Handlers / routeur
// ---------------------------------------------------------------------------

async fn blocking<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> T {
    tokio::task::spawn_blocking(f)
        .await
        .expect("tâche interrompue")
}

async fn index() -> Html<String> {
    Html(blocking(dashboard_html).await)
}

async fn rules(Query(q): Query<RulesQuery>) -> Html<String> {
    Html(blocking(move || rules_html(&q)).await)
}

async fn rule_detail(Query(q): Query<HashMap<String, String>>) -> Response {
    let id = q.get("id").cloned().unwrap_or_default();
    match blocking(move || rule_html(&id)).await {
        Some(html) => Html(html).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Html(page(
                "Introuvable",
                "/rules",
                "<h1>Règle introuvable</h1><p><a href=\"/rules\">← Règles</a></p>",
            )),
        )
            .into_response(),
    }
}

async fn checkers_page() -> Html<String> {
    Html(blocking(checkers_html).await)
}

async fn generate_form() -> Html<String> {
    Html(blocking(|| generate_form_html(None)).await)
}

/// Refuse un POST dont l'`Origin` n'est pas l'hôte local.
fn same_origin(headers: &HeaderMap, port: u16) -> bool {
    match headers.get(header::ORIGIN).and_then(|v| v.to_str().ok()) {
        None => true,
        Some(origin) => [
            format!("http://127.0.0.1:{}", port),
            format!("http://localhost:{}", port),
        ]
        .iter()
        .any(|o| o == origin),
    }
}

async fn generate_post(port: u16, headers: HeaderMap, Form(form): Form<GenerateForm>) -> Response {
    if !same_origin(&headers, port) {
        return (StatusCode::FORBIDDEN, "origine refusée").into_response();
    }
    let (status, html) = blocking(move || generate_result(&form)).await;
    (status, Html(html)).into_response()
}

async fn lint_page(Query(q): Query<LintQuery>) -> Html<String> {
    Html(blocking(move || lint_html(&q)).await)
}

async fn api_rules() -> Json<serde_json::Value> {
    let rules = blocking(|| Rule::load_all().unwrap_or_default()).await;
    Json(serde_json::json!(rules
        .iter()
        .map(|r| serde_json::json!({
            "id": r.id, "name": r.name, "severity": r.severity, "apply": r.apply, "kind": r.kind(),
            "message": r.display_message()
        }))
        .collect::<Vec<_>>()))
}

async fn api_lint(Query(q): Query<LintQuery>) -> Response {
    let path = q.path.clone().unwrap_or_else(|| ".".into());
    let profile = q.profile.clone().filter(|p| !p.is_empty());
    match blocking(move || engine::lint(Path::new(&path), profile.as_deref())).await {
        Ok(r) => Json(serde_json::json!({
            "violations": r.violations, "files_scanned": r.files_scanned, "rules_used": r.rules_used
        }))
        .into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": format!("{:#}", e) })),
        )
            .into_response(),
    }
}

async fn api_sarif(Query(q): Query<LintQuery>) -> Response {
    let path = q.path.clone().unwrap_or_else(|| ".".into());
    let profile = q.profile.clone().filter(|p| !p.is_empty());
    match blocking(move || engine::lint(Path::new(&path), profile.as_deref())).await {
        Ok(r) => (
            [(header::CONTENT_TYPE, "application/sarif+json")],
            crate::sarif::to_sarif(&r.violations),
        )
            .into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": format!("{:#}", e) })),
        )
            .into_response(),
    }
}

pub fn router(port: u16) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/rules", get(rules))
        .route("/rule", get(rule_detail))
        .route("/checkers", get(checkers_page))
        .route(
            "/generate",
            get(generate_form)
                .post(move |h: HeaderMap, f: Form<GenerateForm>| generate_post(port, h, f)),
        )
        .route("/lint", get(lint_page))
        .route("/api/rules", get(api_rules))
        .route("/api/lint", get(api_lint))
        .route("/api/sarif", get(api_sarif))
        .route("/healthz", get(|| async { "ok" }))
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

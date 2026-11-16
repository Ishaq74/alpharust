//! Tests HTTP de la GUI (sans ouvrir de port) via `tower::ServiceExt::oneshot`.

use axum::body::Body;
use axum::http::{header, Method, Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use std::sync::Once;
use tower::ServiceExt;

static INIT: Once = Once::new();
static HOME: std::sync::OnceLock<tempfile::TempDir> = std::sync::OnceLock::new();

/// Racine AlphaRust jetable : règles/profils du dépôt (liens symboliques), `sites/` isolé.
fn setup() {
    INIT.call_once(|| {
        let home = tempfile::tempdir().unwrap();
        let repo = std::env::current_dir().unwrap();
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(repo.join("rules"), home.path().join("rules")).unwrap();
            std::os::unix::fs::symlink(repo.join("profiles"), home.path().join("profiles"))
                .unwrap();
        }
        std::env::set_var("ALPHARUST_HOME", home.path());
        std::env::set_var("ALPHARUST_NO_CACHE", "1");
        HOME.set(home).unwrap();
    });
}

struct Resp {
    status: StatusCode,
    body: String,
    ctype: Option<String>,
    headers: axum::http::HeaderMap,
}

impl Resp {
    fn json(&self) -> Value {
        serde_json::from_str(&self.body)
            .unwrap_or_else(|e| panic!("JSON invalide ({e}) : {}", self.body))
    }
}

async fn call(app: axum::Router, req: Request<Body>) -> Resp {
    setup();
    let res = app.oneshot(req).await.unwrap();
    let status = res.status();
    let headers = res.headers().clone();
    let ctype = headers
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().to_string());
    let body = res.into_body().collect().await.unwrap().to_bytes();
    Resp {
        status,
        body: String::from_utf8_lossy(&body).into_owned(),
        ctype,
        headers,
    }
}

fn app() -> axum::Router {
    setup();
    alpharust::gui::router(3000)
}

async fn get(uri: &str) -> Resp {
    call(app(), Request::get(uri).body(Body::empty()).unwrap()).await
}

fn post_req(uri: &str, body: &Value, origin: Option<&str>) -> Request<Body> {
    let mut b = Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header(header::CONTENT_TYPE, "application/json");
    if let Some(o) = origin {
        b = b.header(header::ORIGIN, o);
    }
    b.body(Body::from(body.to_string())).unwrap()
}

async fn post(uri: &str, body: &Value) -> Resp {
    call(app(), post_req(uri, body, Some("http://127.0.0.1:3000"))).await
}

#[tokio::test]
async fn index_is_a_self_contained_accessible_spa() {
    let r = get("/").await;
    assert_eq!(r.status, StatusCode::OK);
    for needle in [
        "lang=\"fr\"",
        "/assets/app.js",
        "/assets/app.css",
        "skip-link",
        "<main id=\"main\"",
        "aria-label=\"Navigation principale\"",
    ] {
        assert!(r.body.contains(needle), "index sans `{needle}`");
    }
    let csp = r.headers[header::CONTENT_SECURITY_POLICY].to_str().unwrap();
    assert!(csp.contains("default-src 'self'") && csp.contains("script-src 'self'"));
    assert!(
        !r.body.contains("http://") && !r.body.contains("https://"),
        "aucune ressource externe"
    );
    assert_eq!(r.headers["x-frame-options"], "DENY");
}

#[tokio::test]
async fn assets_are_embedded_and_never_use_innerhtml() {
    let js = get("/assets/app.js").await;
    assert_eq!(js.status, StatusCode::OK);
    assert!(js.ctype.unwrap().starts_with("text/javascript"));
    for banned in [
        "innerHTML",
        "outerHTML",
        "insertAdjacentHTML",
        "document.write",
        "eval(",
    ] {
        assert!(!js.body.contains(banned), "app.js contient `{banned}`");
    }
    assert!(js.body.contains("EventSource"), "flux SSE côté client");
    let css = get("/assets/app.css").await;
    assert!(css.ctype.unwrap().starts_with("text/css"));
    for needle in [
        "prefers-reduced-motion",
        "[data-theme=\"light\"]",
        "@media (max-width: 860px)",
        ":focus-visible",
    ] {
        assert!(css.body.contains(needle), "css sans `{needle}`");
    }
    assert_eq!(get("/favicon.svg").await.status, StatusCode::OK);
    assert_eq!(get("/healthz").await.body, "ok");
    assert_eq!(get("/nope").await.status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn meta_describes_frameworks_and_options() {
    let m = get("/api/meta").await.json();
    assert_eq!(m["rules"], 97);
    assert_eq!(m["checkers"], 15);
    assert!(m["profiles"].as_array().unwrap().len() >= 8);
    let fws = m["frameworks"].as_array().unwrap();
    let ids: Vec<&str> = fws.iter().map(|f| f["id"].as_str().unwrap()).collect();
    assert_eq!(ids, ["astro", "laravel", "wagtail", "rust"]);
    let astro = &fws[0]["options"];
    let ints: Vec<&str> = astro["integrations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|i| i["id"].as_str().unwrap())
        .collect();
    for i in ["tailwind", "mdx", "sitemap", "react", "vue", "svelte"] {
        assert!(ints.contains(&i), "{i}");
    }
    assert_eq!(
        astro["package_manager"],
        json!(["npm", "pnpm", "yarn", "bun"])
    );
    assert_eq!(astro["typescript"], json!(["base", "strict", "strictest"]));
}

#[tokio::test]
async fn doctor_reports_every_toolchain() {
    let d = get("/api/doctor").await.json();
    assert_eq!(d.as_array().unwrap().len(), 4);
    for f in d.as_array().unwrap() {
        assert!(f["tools"]
            .as_array()
            .unwrap()
            .iter()
            .any(|t| t["required"] == true));
        assert!(f["ready"].is_boolean());
    }
    let one = get("/api/doctor?framework=rust").await.json();
    assert_eq!(one.as_array().unwrap().len(), 1);
    assert_eq!(
        get("/api/doctor?framework=cobol").await.status,
        StatusCode::BAD_REQUEST
    );
}

#[tokio::test]
async fn plan_previews_official_commands_and_validates() {
    let ok = post(
        "/api/plan",
        &json!({"name":"vitrine","framework":"astro","package_manager":"pnpm","integrations":["tailwind","sitemap"],"site":"https://exemple.com","typescript":"strictest"}),
    )
    .await;
    assert_eq!(ok.status, StatusCode::OK, "{}", ok.body);
    let p = ok.json();
    let cmds: Vec<&str> = p["steps"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|s| s["command"].as_str())
        .collect();
    assert!(
        cmds.iter()
            .any(|c| c.starts_with("pnpm create astro@latest vitrine")),
        "{cmds:?}"
    );
    assert!(
        cmds.iter()
            .any(|c| c.contains("astro add tailwind sitemap --yes")),
        "{cmds:?}"
    );
    assert!(
        p["preview"].as_str().unwrap().contains("create-astro")
            || p["preview"].as_str().unwrap().contains("create astro")
    );
    assert!(p["tools"].as_array().unwrap().len() >= 3);
    assert!(p["target"].as_str().unwrap().ends_with("sites/vitrine"));

    // règles de compatibilité
    let bad = post(
        "/api/plan",
        &json!({"name":"x","framework":"astro","integrations":["sitemap"]}),
    )
    .await;
    assert_eq!(bad.status, StatusCode::UNPROCESSABLE_ENTITY);
    assert!(bad.json()["error"].as_str().unwrap().contains("sitemap"));
    let two = post(
        "/api/plan",
        &json!({"name":"x","framework":"astro","integrations":["react","vue"]}),
    )
    .await;
    assert_eq!(two.status, StatusCode::UNPROCESSABLE_ENTITY);
    let name = post("/api/plan", &json!({"name":"../evil","framework":"rust"})).await;
    assert_eq!(name.status, StatusCode::UNPROCESSABLE_ENTITY);
    let lv = post(
        "/api/plan",
        &json!({"name":"x","framework":"laravel","dark":true}),
    )
    .await;
    assert_eq!(lv.status, StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn client_cannot_choose_the_destination() {
    let r = post(
        "/api/plan",
        &json!({"name":"jail","framework":"rust","path":"/etc/evil"}),
    )
    .await;
    assert_eq!(r.status, StatusCode::OK);
    let target = r.json()["target"].as_str().unwrap().to_string();
    assert!(target.ends_with("sites/jail"), "{target}");
}

#[tokio::test]
async fn post_requires_a_local_origin() {
    let body = json!({"name":"x","framework":"rust"});
    for uri in ["/api/plan", "/api/jobs"] {
        let r = call(app(), post_req(uri, &body, Some("http://evil.example"))).await;
        assert_eq!(r.status, StatusCode::FORBIDDEN, "{uri}");
    }
    let mut req = post_req("/api/plan", &body, None);
    req.headers_mut()
        .insert(header::HOST, "evil.example:3000".parse().unwrap());
    assert_eq!(
        call(app(), req).await.status,
        StatusCode::FORBIDDEN,
        "DNS rebinding"
    );
    let cancel = call(
        app(),
        post_req(
            "/api/jobs/abc/cancel",
            &json!({}),
            Some("http://evil.example"),
        ),
    )
    .await;
    assert_eq!(cancel.status, StatusCode::FORBIDDEN);
    assert_eq!(
        post("/api/jobs/zzz/cancel", &json!({})).await.status,
        StatusCode::NOT_FOUND
    );
}

#[tokio::test]
async fn rules_api_supports_listing_and_detail() {
    let rules = get("/api/rules").await.json();
    assert_eq!(rules.as_array().unwrap().len(), 97);
    let first = &rules[0];
    for k in [
        "id", "name", "category", "severity", "apply", "kind", "message",
    ] {
        assert!(first.get(k).is_some(), "{k}");
    }
    let d = get("/api/rule?id=core/no-eval").await;
    assert_eq!(d.status, StatusCode::OK);
    let d = d.json();
    assert_eq!(d["severity"], "error");
    assert!(d["pattern"].is_string());
    assert!(!d["examples"]["bad"].as_array().unwrap().is_empty());
    assert_eq!(
        get("/api/rule?id=nope/none").await.status,
        StatusCode::NOT_FOUND
    );
    let c = get("/api/checkers").await.json();
    assert_eq!(c.as_array().unwrap().len(), 15);
}

#[tokio::test]
async fn lint_api_returns_counts_violations_and_sarif() {
    let r = get("/api/lint?path=tests/fixtures/violations&profile=strict").await;
    assert_eq!(r.status, StatusCode::OK, "{}", r.body);
    let j = r.json();
    assert!(j["counts"]["error"].as_u64().unwrap() >= 10);
    assert!(j["files_scanned"].as_u64().unwrap() > 0);
    let v = &j["violations"][0];
    for k in ["file", "line", "rule", "rule_id", "message", "severity"] {
        assert!(v.get(k).is_some(), "{k}");
    }
    let clean = get("/api/lint?path=tests/fixtures/clean&profile=strict")
        .await
        .json();
    assert_eq!(clean["violations"].as_array().unwrap().len(), 0);

    let s = get("/api/sarif?path=tests/fixtures/violations&profile=strict").await;
    assert_eq!(s.ctype.as_deref(), Some("application/sarif+json"));
    assert_eq!(s.json()["version"], "2.1.0");

    for bad in ["/etc", "../../..", "/nonexistent/dir"] {
        let r = get(&format!("/api/lint?path={bad}")).await;
        assert_eq!(r.status, StatusCode::BAD_REQUEST, "{bad}");
        assert!(r.json()["error"].is_string());
    }
    assert_eq!(
        get("/api/lint?path=.&profile=nope").await.status,
        StatusCode::BAD_REQUEST
    );
}

/// Cycle complet d'un job : création, concurrence, flux SSE rejouable, historique, projets.
/// Les jobs vivent dans l'état du routeur : un seul routeur (cloné) sert tout le scénario.
#[tokio::test]
async fn generation_job_streams_events_over_sse() {
    let router = app();
    let origin = Some("http://localhost:3000");
    // un crate Rust sans installation ni vérification : `cargo new` + couche + manifeste + lint
    let spec = |name: &str| json!({"name":name,"framework":"rust","install":false,"verify":false,"git":false});

    let created = call(
        router.clone(),
        post_req("/api/jobs", &spec("sse-demo"), origin),
    )
    .await;
    assert_eq!(created.status, StatusCode::ACCEPTED, "{}", created.body);
    let job = created.json();
    let id = job["id"].as_str().unwrap().to_string();
    assert_eq!(job["framework"], "rust");
    assert_eq!(job["status"], "running");

    // concurrence : tant que le job tourne une 2e génération est refusée ; s'il est déjà fini elle est acceptée
    let second = call(
        router.clone(),
        post_req("/api/jobs", &spec("sse-other"), origin),
    )
    .await;
    assert!(
        second.status == StatusCode::CONFLICT || second.status == StatusCode::ACCEPTED,
        "{}",
        second.status
    );

    // le flux se termine avec le job : on le lit en entier
    let uri = format!("/api/jobs/{id}/events");
    let events = call(
        router.clone(),
        Request::get(&uri).body(Body::empty()).unwrap(),
    )
    .await;
    assert_eq!(events.status, StatusCode::OK);
    assert!(events
        .ctype
        .as_deref()
        .unwrap()
        .starts_with("text/event-stream"));
    let sse = events.body;
    assert!(sse.contains("event: end"), "{sse}");
    for t in ["plan", "step_start", "step_end", "log", "done"] {
        assert!(
            sse.contains(&format!("\"type\":\"{t}\"")),
            "événement `{t}` absent : {sse}"
        );
    }
    assert!(sse.contains("\"status\":\"succeeded\""), "{sse}");

    // reprise : `?from=N` rejoue seulement la suite
    let tail = call(
        router.clone(),
        Request::get(format!("{uri}?from=3"))
            .body(Body::empty())
            .unwrap(),
    )
    .await;
    assert!(
        !tail.body.contains("id: 0\n") && !tail.body.contains("id: 2\n"),
        "{}",
        tail.body
    );
    assert!(tail.body.contains("id: 3\n") && tail.body.contains("event: end"));
    let resumed = call(
        router.clone(),
        Request::get(&uri)
            .header("last-event-id", "3")
            .body(Body::empty())
            .unwrap(),
    )
    .await;
    assert!(
        !resumed.body.contains("id: 3\n") && resumed.body.contains("id: 4\n"),
        "Last-Event-ID"
    );

    let list = call(
        router.clone(),
        Request::get("/api/jobs").body(Body::empty()).unwrap(),
    )
    .await
    .json();
    assert!(list
        .as_array()
        .unwrap()
        .iter()
        .any(|j| j["id"] == id.as_str() && j["status"] == "succeeded"));
    let one = call(
        router.clone(),
        Request::get(format!("/api/jobs/{id}"))
            .body(Body::empty())
            .unwrap(),
    )
    .await
    .json();
    assert_eq!(one["status"], "succeeded");
    assert!(one["elapsed_ms"].is_number());

    let project = alpharust::paths::sites_dir().join("sse-demo");
    assert!(
        project.join("Cargo.toml").is_file(),
        "cargo new a bien créé le crate"
    );
    assert!(project.join(".alpharust-project.json").is_file());
    let projects = call(
        router,
        Request::get("/api/projects").body(Body::empty()).unwrap(),
    )
    .await
    .json();
    let p = projects["projects"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["name"] == "sse-demo")
        .unwrap()
        .clone();
    assert_eq!(p["framework"], "rust");
    assert_eq!(p["generated"], true);
}

#[tokio::test]
async fn unknown_job_is_404() {
    assert_eq!(get("/api/jobs/nope").await.status, StatusCode::NOT_FOUND);
    assert_eq!(
        get("/api/jobs/nope/events").await.status,
        StatusCode::NOT_FOUND
    );
}

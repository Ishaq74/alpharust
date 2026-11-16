//! Tests HTTP de la GUI (sans ouvrir de port) via `tower::ServiceExt::oneshot`.

use axum::body::Body;
use axum::http::{header, Method, Request, StatusCode};
use http_body_util::BodyExt;
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

async fn call(req: Request<Body>) -> (StatusCode, String, Option<String>) {
    setup();
    let res = alpharust::gui::router(3000).oneshot(req).await.unwrap();
    let status = res.status();
    let ctype = res
        .headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().to_string());
    let body = res.into_body().collect().await.unwrap().to_bytes();
    (status, String::from_utf8_lossy(&body).into_owned(), ctype)
}

async fn get(uri: &str) -> (StatusCode, String, Option<String>) {
    call(Request::get(uri).body(Body::empty()).unwrap()).await
}

fn post(uri: &str, body: &str, origin: Option<&str>) -> Request<Body> {
    let mut b = Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded");
    if let Some(o) = origin {
        b = b.header(header::ORIGIN, o);
    }
    b.body(Body::from(body.to_string())).unwrap()
}

#[tokio::test]
async fn dashboard_lists_stats_and_navigation() {
    let (status, html, _) = get("/").await;
    assert_eq!(status, StatusCode::OK);
    assert!(
        html.contains("AlphaRust")
            && html.contains("règles")
            && html.contains("checkers sémantiques")
    );
    for link in ["/rules", "/generate", "/lint", "/checkers"] {
        assert!(
            html.contains(&format!("href=\"{}\"", link)),
            "lien {} absent",
            link
        );
    }
    assert!(html.contains("strict"), "profils listés");
}

#[tokio::test]
async fn rules_page_filters_and_detail() {
    let (_, all, _) = get("/rules").await;
    assert!(all.contains("core/no-eval") && all.contains("domains/cloud/aws-access-key-id"));
    let (_, filtered, _) = get("/rules?q=stripe").await;
    assert!(filtered.contains("stripe-secret-key-env") || filtered.contains("payments/stripe"));
    assert!(!filtered.contains("core/no-eval"));
    let (_, errors, _) = get("/rules?severity=info").await;
    assert!(!errors.contains("core/no-eval"), "no-eval est une erreur");
    let (status, detail, _) = get("/rule?id=core/no-eval").await;
    assert_eq!(status, StatusCode::OK);
    assert!(detail.contains("Pattern") && detail.contains("eval"));
    let (status, _, _) = get("/rule?id=../../etc/passwd").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn lint_page_shows_real_results() {
    let (status, html, _) = get("/lint?path=tests/fixtures/violations&profile=core").await;
    assert_eq!(status, StatusCode::OK);
    assert!(
        html.contains("violation(s)")
            && html.contains("core/no-eval")
            && html.contains("badge error")
    );
    let (_, clean, _) = get("/lint?path=tests/fixtures/clean").await;
    assert!(clean.contains("Aucune violation"));
    let (_, bad, _) = get("/lint?path=/inexistant").await;
    assert!(bad.contains("introuvable"));
    let (_, form, _) = get("/lint").await;
    assert!(form.contains("<form") && form.contains("Lancer le lint"));
}

#[tokio::test]
async fn api_endpoints_return_json_and_sarif() {
    let (status, body, ctype) =
        get("/api/lint?path=tests/fixtures/violations&profile=strict").await;
    assert_eq!(status, StatusCode::OK);
    assert!(ctype.unwrap().contains("json"));
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert!(json["violations"].as_array().unwrap().len() > 5);
    let (_, body, ctype) = get("/api/sarif?path=tests/fixtures/violations&profile=core").await;
    assert!(ctype.unwrap().contains("sarif"));
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&body).unwrap()["version"],
        "2.1.0"
    );
    let (_, body, _) = get("/api/rules").await;
    assert!(
        serde_json::from_str::<serde_json::Value>(&body)
            .unwrap()
            .as_array()
            .unwrap()
            .len()
            >= 90
    );
    let (status, _, _) = get("/api/lint?path=/inexistant").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(get("/healthz").await.1, "ok");
}

#[tokio::test]
async fn html_is_escaped() {
    let (_, html, _) = get("/lint?path=%3Cscript%3Ealert(1)%3C/script%3E").await;
    assert!(!html.contains("<script>alert(1)"), "injection non échappée");
    assert!(html.contains("&lt;script&gt;"));
    let (_, html, _) = get("/rules?q=%22%3E%3Cimg%20src=x%3E").await;
    assert!(!html.contains("\"><img src=x>"));
}

#[tokio::test]
async fn generate_creates_projects_and_validates_input() {
    let (_, form, _) = get("/generate").await;
    assert!(form.contains("Wagtail") && form.contains("Astro") && form.contains("Laravel"));

    let (status, html, _) = call(post(
        "/generate",
        "kind=astro&name=gui-demo",
        Some("http://127.0.0.1:3000"),
    ))
    .await;
    assert_eq!(status, StatusCode::OK, "{}", html);
    assert!(html.contains("créé dans sites/gui-demo") && html.contains("package.json"));
    assert!(alpharust::paths::sites_dir()
        .join("gui-demo/package.json")
        .exists());

    let (status, html, _) = call(post("/generate", "kind=astro&name=gui-demo", None)).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(html.contains("existe déjà"));

    let (status, html, _) = call(post("/generate", "kind=astro&name=..%2Fevil", None)).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(html.contains("invalide"));
    assert!(!alpharust::paths::sites_dir().join("../evil").exists());

    let (status, _, _) = call(post("/generate", "kind=nope&name=x", None)).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);

    let (status, _, _) = call(post(
        "/generate",
        "kind=astro&name=csrf",
        Some("http://evil.example"),
    ))
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert!(!alpharust::paths::sites_dir().join("csrf").exists());

    let (_, listing, _) = get("/generate").await;
    assert!(listing.contains("sites/gui-demo"));
}

#[tokio::test]
async fn checkers_page_lists_all_checkers() {
    let (_, html, _) = get("/checkers").await;
    for (name, _) in alpharust::checkers::catalog() {
        assert!(html.contains(name), "{} absent", name);
    }
    assert!(html.contains("languages/rust/no-unsafe"));
}

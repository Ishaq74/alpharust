use axum::{
    extract::Form,
    response::Html,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::net::SocketAddr;

#[derive(Deserialize)]
struct GenerateForm {
    kind: String,
    name: String,
}

pub async fn run() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(|| async { Html(INDEX) }))
        .route("/rules", get(rules))
        .route("/generate", get(|| async { Html(GENERATE) }).post(do_generate))
        .route("/lint", get(|| async { Html(LINT) }).post(do_lint));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("GUI: http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

const INDEX: &str = r#"<h1>AlphaRust</h1><p><a href='/rules'>Rules</a> | <a href='/generate'>Generate</a> | <a href='/lint'>Lint</a></p>"#;
const GENERATE: &str = r#"<form method='post'><select name='kind'><option value='wagtail'>Wagtail</option><option value='astro'>Astro</option><option value='laravel'>Laravel</option></select><input name='name' required><button>Go</button></form>"#;
const LINT: &str = r#"<form method='post'><input name='path' value='.'><button>Lint</button></form>"#;

async fn rules() -> Html<String> {
    let rules = crate::parser::Rule::load_all().unwrap_or_default();
    let mut s = String::from("<ul>");
    for r in rules {
        s.push_str(&format!("<li>{} [{}]</li>", r.name, r.severity));
    }
    s.push_str("</ul>");
    Html(s)
}

async fn do_generate(Form(f): Form<GenerateForm>) -> Html<String> {
    let p = std::path::PathBuf::from("sites").join(&f.name);
    let r = match f.kind.as_str() {
        "wagtail" => crate::generators::wagtail::generate(&p, &f.name),
        "astro" => crate::generators::astro::generate(&p, &f.name),
        "laravel" => crate::generators::laravel::generate(&p, &f.name),
        _ => Err(anyhow::anyhow!("bad kind")),
    };
    Html(format!("<p>{:?}</p>", r.map(|_| "ok")))
}

async fn do_lint(Form(f): Form<HashMap<String, String>>) -> Html<String> {
    let path = f.get("path").map(|s| s.as_str()).unwrap_or(".");
    let rules = crate::parser::Rule::load_all().unwrap_or_default();
    let v = crate::engine::runner::run(&rules, std::path::Path::new(path)).unwrap_or_default();
    Html(format!("<p>{} violations</p>", v.len()))
}

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
        .route("/", get(index))
        .route("/rules", get(rules_handler))
        .route("/generate", get(generate_form).post(generate_handler))
        .route("/lint", get(lint_form).post(lint_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("AlphaRust GUI running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn index() -> Html<&'static str> {
    Html(
        r#"<!DOCTYPE html>
<html><head><meta charset="utf-8"><title>AlphaRust</title>
<style>body{font-family:system-ui;background:#111;color:#eee;padding:2rem}a{color:#4af}</style>
</head><body>
<h1>AlphaRust</h1>
<p><a href="/rules">Règles</a> · <a href="/generate">Générer</a> · <a href="/lint">Lint</a></p>
</body></html>"#,
    )
}

async fn rules_handler() -> Html<String> {
    match crate::parser::Rule::load_all() {
        Ok(rules) => {
            let mut html = String::from("<h1>Règles</h1><ul>");
            for r in rules {
                html.push_str(&format!(
                    "<li><strong>{}</strong> <em>[{}]</em> {}</li>",
                    r.name, r.severity, r.apply
                ));
            }
            html.push_str("</ul><p><a href='/'>Retour</a></p>");
            Html(html)
        }
        Err(e) => Html(format!("<p>Erreur: {}</p>", e)),
    }
}

async fn generate_form() -> Html<&'static str> {
    Html(
        r#"<h1>Générer un projet</h1>
<form method="post" action="/generate">
  <select name="kind">
    <option value="wagtail">Wagtail</option>
    <option value="astro">Astro</option>
    <option value="laravel">Laravel</option>
  </select>
  <input type="text" name="name" placeholder="nom-du-projet" required>
  <button type="submit">Générer dans sites/</button>
</form>
<p><a href="/">Retour</a></p>"#,
    )
}

async fn generate_handler(Form(form): Form<GenerateForm>) -> Html<String> {
    let path = std::path::PathBuf::from("sites").join(&form.name);
    let result = match form.kind.as_str() {
        "wagtail" => crate::generators::wagtail::generate(&path, &form.name),
        "astro" => crate::generators::astro::generate(&path, &form.name),
        "laravel" => crate::generators::laravel::generate(&path, &form.name),
        _ => Err(anyhow::anyhow!("type inconnu")),
    };

    let message = match result {
        Ok(_) => format!("Projet '{}' créé dans sites/{}", form.name, form.name),
        Err(e) => format!("Erreur: {}", e),
    };

    Html(format!("<p>{}</p><p><a href='/'>Retour</a></p>", message))
}

async fn lint_form() -> Html<&'static str> {
    Html(
        r#"<h1>Lint</h1>
<form method="post" action="/lint">
  <input type="text" name="path" value="." placeholder="chemin">
  <button type="submit">Lancer le lint</button>
</form>"#,
    )
}

async fn lint_handler(Form(params): Form<HashMap<String, String>>) -> Html<String> {
    let path = params.get("path").cloned().unwrap_or_else(|| ".".to_string());
    let rules = crate::parser::Rule::load_all().unwrap_or_default();
    let violations = crate::engine::runner::run(&rules, std::path::Path::new(&path))
        .unwrap_or_default();

    let mut html = format!("<h1>Résultats ({} violation(s))</h1><ul>", violations.len());
    for v in &violations {
        html.push_str(&format!(
            "<li>{}:{} — <strong>{}</strong></li>",
            v.file, v.line, v.rule
        ));
    }
    html.push_str("</ul><p><a href='/'>Retour</a></p>");
    Html(html)
}

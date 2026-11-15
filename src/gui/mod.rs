use axum::{
    extract::Form,
    response::Html,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Deserialize)]
struct GenerateForm {
    kind: String,
    name: String,
}

pub async fn run() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(index))
        .route("/rules", get(rules_page))
        .route("/generate", get(generate_form).post(generate_action))
        .route("/lint", get(lint_form).post(lint_action));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("AlphaRust GUI → http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn index() -> Html<&'static str> {
    Html(r#"<!DOCTYPE html>
<html lang="fr">
<head>
<meta charset="utf-8">
<title>AlphaRust</title>
<style>
body{font-family:system-ui;background:#111;color:#eee;margin:0;padding:2rem}
a{color:#6cf} h1{color:#fff}
.card{background:#1a1a1a;padding:1.5rem;border-radius:8px;margin:1rem 0}
button,input,select{padding:.5rem;margin:.3rem;background:#222;color:#eee;border:1px solid #444;border-radius:4px}
</style>
</head>
<body>
<h1>AlphaRust</h1>
<p>Fondation intégrale — CLI / TUI / GUI</p>
<div class="card">
<a href="/rules">→ Voir les règles</a><br>
<a href="/generate">→ Générer un projet</a><br>
<a href="/lint">→ Lancer un lint</a>
</div>
</body>
</html>"#)
}

async fn rules_page() -> Html<String> {
    let rules = crate::parser::Rule::load_all().unwrap_or_default();
    let mut html = String::from("<h1>Règles</h1><ul>");
    for r in rules {
        html.push_str(&format!(" <li><b>{}</b> [{}] — {}</li>", r.name, r.severity, r.apply));
    }
    html.push_str("</ul><a href='/'>Retour</a>");
    Html(html)
}

async fn generate_form() -> Html<&'static str> {
    Html(r#"<h1>Générer un projet</h1>
<form method="post">
<select name="kind">
<option value="wagtail">Wagtail</option>
<option value="astro">Astro</option>
<option value="laravel">Laravel</option>
</select>
<input name="name" placeholder="nom-du-projet" required>
<button type="submit">Générer dans sites/</button>
</form>
<a href="/">Retour</a>"#)
}

async fn generate_action(Form(form): Form<GenerateForm>) -> Html<String> {
    let path = std::path::PathBuf::from("sites").join(&form.name);
    let result = match form.kind.as_str() {
        "wagtail" => crate::generators::wagtail::generate(&path, &form.name),
        "astro" => crate::generators::astro::generate(&path, &form.name),
        "laravel" => crate::generators::laravel::generate(&path, &form.name),
        _ => Err(anyhow::anyhow!("unknown kind")),
    };
    let msg = match result {
        Ok(_) => format!("✓ Projet {} généré dans sites/{}", form.name, form.name),
        Err(e) => format!("Erreur: {}", e),
    };
    Html(format!("<p>{}</p><a href='/'>Retour</a>", msg))
}

async fn lint_form() -> Html<&'static str> {
    Html(r#"<h1>Lint</h1>
<form method="post">
<input name="path" value="." placeholder="chemin">
<button type="submit">Lancer</button>
</form>"#)
}

async fn lint_action(Form(form): Form<std::collections::HashMap<String, String>>) -> Html<String> {
    let path = form.get("path").cloned().unwrap_or_else(|| ".".into());
    let rules = crate::parser::Rule::load_all().unwrap_or_default();
    let violations = crate::engine::runner::run(&rules, std::path::Path::new(&path)).unwrap_or_default();
    let mut html = format!("<h1>Résultats ({} violations)</h1><ul>", violations.len());
    for v in violations {
        html.push_str(&format!(" <li>{}:{} — {}</li>", v.file, v.line, v.rule));
    }
    html.push_str("</ul><a href='/'>Retour</a>");
    Html(html)
}

use axum::{routing::get, Router};
use std::net::SocketAddr;

pub async fn run() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(index))
        .route("/rules", get(rules_page))
        .route("/generate", get(generate_page));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("AlphaRust GUI running on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn index() -> &'static str {
    r#"<!DOCTYPE html>
<html>
<head><title>AlphaRust</title>
<style>
body { font-family: system-ui; background: #0f0f0f; color: #eee; margin: 2rem; }
a { color: #4ea1ff; }
</style>
</head>
<body>
<h1>AlphaRust GUI</h1>
<p>Interface web style forge, intégrée.</p>
<ul>
<li><a href="/rules">Règles</a></li>
<li><a href="/generate">Générer un projet</a></li>
</ul>
</body>
</html>"#
}

async fn rules_page() -> &'static str {
    "<h1>Rules</h1><p>Liste des règles (à brancher sur le moteur)</p>"
}

async fn generate_page() -> &'static str {
    "<h1>Generate</h1><p>Formulaire de génération (Wagtail / Astro / Laravel)</p>"
}

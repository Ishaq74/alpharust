#[tokio::main]
async fn main() {
    match alpharust::cli::run().await {
        Ok(code) => std::process::exit(code),
        Err(e) => {
            eprintln!("Erreur : {e:#}");
            std::process::exit(2);
        }
    }
}

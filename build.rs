//! Les modèles de couches sont embarqués à la compilation (`include_dir!`) :
//! on demande à Cargo de recompiler dès qu'un fichier de `templates/` change.
fn main() {
    println!("cargo:rerun-if-changed=templates");
    println!("cargo:rerun-if-changed=build.rs");
}

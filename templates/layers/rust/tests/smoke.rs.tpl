@@meta when=bin@@
use std::process::Command;

#[test]
fn binary_runs_successfully() {
    let out = Command::new(env!("CARGO_BIN_EXE_@@folder@@"))
        .output()
        .expect("le binaire doit se lancer");
    assert!(
        out.status.success(),
        "code de sortie : {:?}",
        out.status.code()
    );
}

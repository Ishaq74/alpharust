use anyhow::Result;
use std::fs;
use std::path::Path;

/// Génère des tests unitaires squelettes pour un langage donné.
pub fn generate_unit_tests(root: &Path, lang: &str) -> Result<Vec<String>> {
    let mut created = Vec::new();
    let test_dir = root.join("tests");
    fs::create_dir_all(&test_dir).ok();

    match lang {
        "rust" => {
            let f = test_dir.join("unit_tests.rs");
            fs::write(
                &f,
                "// Généré par AlphaRust — tests unitaires squelettes\n#[cfg(test)]\nmod tests {\n    #[test]\n    fn placeholder() {\n        assert!(true);\n    }\n}\n",
            )
            .ok();
            created.push(f.display().to_string());
        }
        "python" => {
            let f = test_dir.join("test_unit.py");
            fs::write(
                &f,
                "# Généré par AlphaRust — tests unitaires squelettes\ndef test_placeholder():\n    assert True\n",
            )
            .ok();
            created.push(f.display().to_string());
        }
        "php" => {
            let f = test_dir.join("UnitTest.php");
            fs::write(
                &f,
                "<?php\n// Généré par AlphaRust — tests unitaires squelettes\nuse PHPUnit\\Framework\\TestCase;\n\nclass UnitTest extends TestCase\n{\n    public function testPlaceholder(): void\n    {\n        $this->assertTrue(true);\n    }\n}\n",
            )
            .ok();
            created.push(f.display().to_string());
        }
        _ => {
            let f = test_dir.join("README.md");
            fs::write(&f, format!("# Tests {}\n\nÀ compléter.\n", lang)).ok();
            created.push(f.display().to_string());
        }
    }
    Ok(created)
}

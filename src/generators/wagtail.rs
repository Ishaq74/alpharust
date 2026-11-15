use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn generate(path: &Path, name: &str) -> Result<()> {
    fs::create_dir_all(path.join("app"))?;
    fs::create_dir_all(path.join("templates"))?;

    fs::write(
        path.join("requirements.txt"),
        "Django>=5.0\nwagtail>=6.0\n",
    )?;

    fs::write(
        path.join("manage.py"),
        "#!/usr/bin/env python\nimport os\nimport sys\n\nif __name__ == '__main__':\n    os.environ.setdefault('DJANGO_SETTINGS_MODULE', 'app.settings')\n    from django.core.management import execute_from_command_line\n    execute_from_command_line(sys.argv)\n",
    )?;

    println!("✓ Projet Wagtail '{}' généré", name);
    Ok(())
}

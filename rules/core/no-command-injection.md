---
name: no-command-injection
apply: "*"
severity: error
pattern: "\\b(os\\.system|shell_exec|passthru|popen|proc_open|child_process\\.exec(Sync)?)\\s*\\(|\\bsystem\\s*\\(|shell\\s*=\\s*True"
examples:
  bad:
    - "os.system(cmd)"
    - "subprocess.run(c, shell=True)"
    - "system($cmd);"
    - "child_process.execSync(x)"
  good:
    - "subprocess.run([\"ls\", \"-l\"])"
    - "ecosystem(x)"
    - "Command::new(\"ls\")"
---

# Éviter l’injection de commandes

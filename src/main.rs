use std::{
    env,
    path::Path,
    process::{self, Command},
};

fn bwrap_available() -> bool {
    Command::new("bwrap")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn canonical_workspace(path: &str) -> Result<String, String> {
    let canonical = Path::new(path)
        .canonicalize()
        .map_err(|error| format!("failed to resolve workspace '{path}': {error}"))?;
    if !canonical.is_dir() {
        return Err(format!("workspace '{}' is not a directory", canonical.display()));
    }
    canonical
        .to_str()
        .map(str::to_owned)
        .ok_or_else(|| "workspace path is not valid UTF-8".to_owned())
}

fn sandbox_args(workspace: &str, command: &[String]) -> Vec<String> {
    let mut args = vec![
        "--die-with-parent".to_owned(),
        "--new-session".to_owned(),
        "--unshare-all".to_owned(),
        "--clearenv".to_owned(),
        "--setenv".to_owned(),
        "PATH".to_owned(),
        "/usr/local/bin:/usr/bin:/bin".to_owned(),
        "--setenv".to_owned(),
        "HOME".to_owned(),
        "/workspace".to_owned(),
        "--proc".to_owned(),
        "/proc".to_owned(),
        "--dev".to_owned(),
        "/dev".to_owned(),
        "--tmpfs".to_owned(),
        "/tmp".to_owned(),
    ];

    for host_path in ["/usr", "/bin", "/lib", "/lib64", "/etc"] {
        if Path::new(host_path).exists() {
            args.push("--ro-bind".to_owned());
            args.push(host_path.to_owned());
            args.push(host_path.to_owned());
        }
    }

    args.extend([
        "--bind".to_owned(),
        workspace.to_owned(),
        "/workspace".to_owned(),
        "--chdir".to_owned(),
        "/workspace".to_owned(),
        "--".to_owned(),
    ]);
    args.extend(command.iter().cloned());
    args
}

fn display_arg(value: &str) -> String {
    if value
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || "-._/:=".contains(ch))
    {
        return value.to_owned();
    }
    format!("'{0}'", value.replace('\'', "'\\''"))
}

fn parse_run(args: &[String]) -> Result<(&str, &[String]), String> {
    if args.len() < 4 || args[0] != "run" || args[2] != "--" {
        return Err("expected 'run <WORKSPACE> -- <COMMAND> [ARGS...]'".to_owned());
    }
    Ok((&args[1], &args[3..]))
}

fn run_sandbox(workspace: &str, command: &[String], dry_run: bool) -> Result<i32, String> {
    if !cfg!(target_os = "linux") {
        return Err("SafeWorkspace currently supports Linux only".to_owned());
    }
    if command.is_empty() {
        return Err("no command supplied after '--'".to_owned());
    }
    if !bwrap_available() {
        return Err(
            "bubblewrap ('bwrap') is required for the current isolation backend and was not found"
                .to_owned(),
        );
    }

    let workspace = canonical_workspace(workspace)?;
    let args = sandbox_args(&workspace, command);

    if dry_run {
        println!(
            "bwrap {}",
            args.iter()
                .map(|arg| display_arg(arg))
                .collect::<Vec<_>>()
                .join(" ")
        );
        return Ok(0);
    }

    let status = Command::new("bwrap")
        .args(&args)
        .status()
        .map_err(|error| format!("failed to launch bubblewrap: {error}"))?;
    Ok(status.code().unwrap_or(1))
}

fn help() {
    println!(
        "SafeWorkspace 0.1.0-dev\n\nUSAGE:\n  safeworkspace status\n  safeworkspace plan <WORKSPACE> -- <COMMAND> [ARGS...]\n  safeworkspace run <WORKSPACE> -- <COMMAND> [ARGS...]\n\nThe current Linux backend uses bubblewrap, makes the selected workspace writable at /workspace, exposes core system paths read-only, clears the environment, provides a temporary /tmp, and unshares network and other namespaces by default."
    );
}

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() || matches!(args[0].as_str(), "--help" | "-h") {
        help();
        return;
    }
    if matches!(args[0].as_str(), "--version" | "-V") {
        println!("safeworkspace 0.1.0-dev");
        return;
    }

    if args.len() == 1 && args[0] == "status" {
        if cfg!(target_os = "linux") && bwrap_available() {
            println!("READY: Linux bubblewrap backend is available");
        } else {
            println!("UNAVAILABLE: Linux bubblewrap backend is not available");
            process::exit(3);
        }
        return;
    }

    let dry_run = args.first().map(String::as_str) == Some("plan");
    if dry_run {
        args[0] = "run".to_owned();
    }
    let (workspace, command) = match parse_run(&args) {
        Ok(parsed) => parsed,
        Err(error) => {
            eprintln!("safeworkspace: {error}");
            process::exit(2);
        }
    };

    match run_sandbox(workspace, command, dry_run) {
        Ok(code) => process::exit(code),
        Err(error) => {
            eprintln!("safeworkspace: {error}");
            process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{display_arg, parse_run, sandbox_args};

    #[test]
    fn plan_unshares_namespaces_and_mounts_workspace() {
        let command = vec!["sh".to_owned(), "-c".to_owned(), "pwd".to_owned()];
        let args = sandbox_args("/tmp/project", &command);
        assert!(args.iter().any(|arg| arg == "--unshare-all"));
        assert!(args.windows(3).any(|window| {
            window == ["--bind", "/tmp/project", "/workspace"]
        }));
    }

    #[test]
    fn parser_requires_explicit_separator() {
        let args = vec![
            "run".to_owned(),
            "/tmp/project".to_owned(),
            "--".to_owned(),
            "true".to_owned(),
        ];
        let (workspace, command) = parse_run(&args).expect("valid arguments");
        assert_eq!(workspace, "/tmp/project");
        assert_eq!(command, &["true"]);
    }

    #[test]
    fn display_quotes_spaces() {
        assert_eq!(display_arg("hello world"), "'hello world'");
    }
}

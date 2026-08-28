use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() || args[0] == "--help" || args[0] == "-h" {
        println!("SafeWorkspace 0.1.0-dev\n\nUSAGE:\n  safeworkspace status\n\nIsolation is not enabled in the current development scaffold.");
        return;
    }
    if args[0] == "--version" || args[0] == "-V" {
        println!("safeworkspace 0.1.0-dev");
        return;
    }
    if args.len() == 1 && args[0] == "status" {
        println!("SafeWorkspace is in early development; no sandbox is active yet.");
        return;
    }
    eprintln!("safeworkspace: unsupported command in the current development scaffold");
    process::exit(2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn package_identity_is_stable() {
        assert_eq!(env!("CARGO_PKG_NAME"), "safeworkspace");
    }
}

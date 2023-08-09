use std::{
    path::Path,
    process::{Command, ExitStatus},
};

pub fn open_file_or_program(path: &str, args: &[&str]) -> std::io::Result<ExitStatus> {
    let ext = Path::new(path).extension().and_then(|s| s.to_str());
    match ext {
        Some("exe") => Command::new(path).args(args).status(),
        _ => {
            if cfg!(target_os = "windows") {
                let mut cmd_args = vec!["/c", "start", path];
                cmd_args.extend(args);
                Command::new("cmd").args(cmd_args).status()
            } else if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
                let mut open_args = vec![path];
                open_args.extend(args);
                Command::new("open").args(open_args).status()
            } else {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Unsupported OS",
                ));
            }
        }
    }
}

pub fn execute_command(command: &str, args: &[&str]) -> std::io::Result<ExitStatus> {
    let mut cmd_args = vec!["/c", command];
    cmd_args.extend(args);
    Command::new("cmd").args(cmd_args).status()
}

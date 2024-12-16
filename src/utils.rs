use anyhow::{Context, Result};
use std::{
    env,
    path::PathBuf,
    process::{ChildStdout, Command, Stdio},
};

pub fn default_config_dir() -> Result<PathBuf> {
    let xdg_dirs = xdg::BaseDirectories::new()?;
    let config_dir = xdg_dirs.get_config_home();

    Ok(config_dir.join("h"))
}

pub fn man_page_exists(args: &[String]) -> Result<bool> {
    Ok(Command::new("man")
        .arg("-w")
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?
        .success())
}

pub fn args_to_cmd(args: &[String]) -> Command {
    let mut cmd = Command::new(&args[0]);
    if args.len() > 1 {
        cmd.args(&args[1..]);
    }

    cmd
}

pub fn pipe_to_pager<T>(output: T, pager: Option<String>) -> Result<()> where Stdio: From<T> {
    let pager = match pager {
        Some(p) => p,
        None => env::var("PAGER").unwrap_or_else(|_| "/usr/bin/less".to_string()),
    };

    Command::new(&pager)
        .stdin(Stdio::from(output))
        .status()
        .context(format!("Failed to pipe help to pager ({pager})"))?;

    Ok(())
}

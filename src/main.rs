use std::{
    env,
    os::unix::process::CommandExt,
    process::{Command, Stdio},
};

use anyhow::{anyhow, bail, Result};
use clap::Parser;

mod app;
use app::App;

fn main() -> Result<()> {
    let app = App::parse();

    let man_page_exists = Command::new("man")
        .arg("-w")
        .args(&app.cmd)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?
        .success();

    if man_page_exists {
        Command::new("man").args(&app.cmd).exec();
    } else {
        let pager = env::var("PAGER").unwrap_or_else(|_| "less".to_string());
        let cmd_help = Command::new(app.cmd.join(""))
            .arg("--help")
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|_| anyhow!("Could not get help for '{}'", &app.cmd.join(" ")))?;

        if let Some(help_text) = cmd_help.stdout {
            Command::new(&pager)
                .stdin(Stdio::from(help_text))
                .status()?;
        } else {
            bail!("No help available for '{}'", &app.cmd.join(" "))
        }
    }

    Ok(())
}

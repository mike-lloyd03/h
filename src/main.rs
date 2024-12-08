use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

use anyhow::{anyhow, bail, Result};
use clap::Parser;

mod app;
mod config;
mod utils;
mod wrapper;

use app::App;
use config::Config;
use utils::{args_to_cmd, default_config_dir, man_page_exists, pipe_to_pager};

fn main() -> Result<()> {
    let app = App::parse();

    let config_dir = match app.config_dir {
        Some(c) => {
            let p = PathBuf::from(c);
            if !p.exists() {
                bail!("The provided config directory does not exist")
            }
            p
        }
        None => default_config_dir()?,
    };

    let config = Config::load(&config_dir)?;

    if let Some(wrappers) = config.wrappers {
        for wrapper in wrappers {
            if wrapper.clone().matches_args(&app.cmd)? {
                let replacement_args = wrapper.clone().parse_replacement(&app.cmd)?;

                let mut cmd = args_to_cmd(&replacement_args);

                if wrapper.use_pager {
                    let help_text = cmd.stdout(Stdio::piped()).spawn()?;

                    match help_text.stdout {
                        Some(o) => pipe_to_pager(o)?,
                        None => todo!(),
                    }
                } else {
                    cmd.status()?;
                }

                return Ok(());
            }
        }
    }

    if man_page_exists(&app.cmd)? {
        Command::new("man").args(&app.cmd).status()?;
    } else {
        let mut help_cmd = app.cmd.clone();
        help_cmd.push("--help".into());

        let help_text = args_to_cmd(&help_cmd)
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|_| anyhow!("Could not get help for '{}'", &app.cmd.join(" ")))?;

        if let Some(help_text) = help_text.stdout {
            pipe_to_pager(help_text)?;
        } else {
            bail!("No help available for '{}'", &app.cmd.join(" "))
        }
    }

    Ok(())
}

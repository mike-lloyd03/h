use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "h")]
#[command(about = "Easily get help for any command")]
#[command(version, about, long_about = None)]
pub struct App {
    #[arg(help = "The command to get help for")]
    pub cmd: Vec<String>,

    #[arg(short, long, help = "Alternate config directory to use")]
    pub config_dir: Option<String>,
}

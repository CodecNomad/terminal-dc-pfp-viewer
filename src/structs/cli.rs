use clap::Parser;

#[derive(Parser)]
#[command(about = "Draws an image from a http/s link into your terminal", long_about = None, version)]
pub struct Cli {
    #[arg(long)]
    pub id: Option<String>,
}

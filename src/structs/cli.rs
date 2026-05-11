use clap::Parser;

#[derive(Parser)]
#[command(
    about = "Display a Discord user's avatar from an ID in your terminal",
    version
)]
pub struct Cli {
    #[arg(long)]
    pub id: Option<String>,
}

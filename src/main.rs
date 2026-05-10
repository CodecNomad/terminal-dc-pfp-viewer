use anyhow::{Context, Result};
use clap::Parser;
use inquire::Text;
use serde::{Deserialize, Serialize};
use serde_json::json;
use viuer::Config;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub global_name: String,
    pub display_name: String,
    pub avatar_url: String,
    pub avatar_download_url: String,
    pub avatar_webp_url: String,
    pub avatar_animated: bool,
    pub banner_url: Option<String>,
    pub banner_download_url: Option<String>,
    pub banner_animated: bool,
    pub accent_color: Option<String>,
    pub banner_color: Option<String>,
    pub public_flags: i32,
    pub created_at: String,
    pub mention: String,
    pub is_bot: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub user: User,
}
fn get_url_of_user(id: String) -> Result<String> {
    let api_response = reqwest::blocking::Client::new()
        .post("https://www.discordpfp.gg/api/discordlookup")
        .body(json!({"discordId": id}).to_string())
        .send()
        .context("Failed to get response from lookup")?;

    let json_response = api_response
        .json::<Root>()
        .context("Failed to turn response into json")?;

    Ok(json_response.user.avatar_download_url)
}

#[derive(Parser)]
#[command(about = "Draws an image from a http/s link into your terminal", long_about = None, version)]
struct Cli {
    #[arg(long)]
    id: Option<String>,
}

fn parse_cli() -> Result<String> {
    let parsed_cli = Cli::parse();

    parsed_cli.id.map_or_else(
        || {
            Text::new("What's the ID of the user?")
                .prompt()
                .context("Failed to ask image URL")
        },
        Ok,
    )
}
fn main() -> Result<()> {
    let id = parse_cli()?;
    let url = get_url_of_user(id)?;

    let image_data = reqwest::blocking::get(url)
        .context("Failed to get image from URL")?
        .bytes()?;

    let image =
        image::load_from_memory(&image_data).context("Failed to construct image from raw bytes")?;

    let _ = clearscreen::clear();

    viuer::print(&image, &Config::default()).context("Failed to print image to terminal")?;

    Ok(())
}

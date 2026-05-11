mod structs;

use anyhow::{Context, Result};
use clap::Parser;
use inquire::Text;
use serde_json::json;
use viuer::Config;

use crate::structs::{api_layout::Root, cli::Cli};

fn get_url_of_user(id: String) -> Result<String> {
    let number_id = id
        .parse::<u64>()
        .context("Failed to conver string ID to u64 ID, invalid ID.")?;

    // A tiny bit smaller than first actual ID
    if number_id < 0x2386F26FC10000 {
        return Err(anyhow::anyhow!("Invalid user ID"));
    }

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

    clearscreen::clear().ok();

    viuer::print(&image, &Config::default()).context("Failed to print image to terminal")?;

    Ok(())
}

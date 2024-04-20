use crate::{structs::apex::{ApexError, ApexPlayer}, Context, Error};
use poise::serenity_prelude::{self, Color};
use reqwest::{StatusCode, Url};

impl ApexPlayer {
    async fn get(api_key: &String, id: &String, platform: &String) -> Result<Self, Error> {
        let url = format!(
            "https://api.mozambiquehe.re/bridge?auth={}&player={}&platform={}",
            api_key, id, platform
        );

        let url = Url::parse(&*url)?;
        let response = reqwest::get(url).await?;
 
        match response.status() {
            StatusCode::OK => (),
            _ => {
                let error_response = response.json::<ApexError>().await?;

                return Err(error_response.error.into())
            }
        };

        let res = response.json::<ApexPlayer>().await?;

        Ok(res)
    }
}

/// Fetch a player's information
#[poise::command(slash_command, prefix_command, track_edits, user_cooldown = 1)]
pub async fn player(
    ctx: Context<'_>,
    #[description = "Player id"] id: String,
    #[description = "Player platform"] platform: String,
) -> Result<(), Error> {
    let api_key = std::env::var("API_KEY").expect("No api key?");
    let response = ApexPlayer::get(&api_key, &id, &platform).await?;

    let status = match response.realtime.is_online {
    1 => "Online",
    0 => "Offline",
    _ => "Offline",
};
   

    ctx.send(
        poise::CreateReply::default()
        .content("Data fetched")
        .embed(
            serenity_prelude::CreateEmbed::new()
            .color(Color::new(0xda292a))
            .title(response.global.name)
            .description(
                format!("LVL {} | Plays on {}", response.global.level, response.global.platform)
            )
            .field("In-game status", status, false)
            .thumbnail(response.global.avatar)
            .footer(
                serenity_prelude::CreateEmbedFooter::new(response.global.uid)
            )
        )
    ).await?;

    Ok(())
}


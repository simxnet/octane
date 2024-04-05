use crate::{Error, Context, structs::apex::ApexPlayer};
use reqwest::Url;

impl ApexPlayer {
    async fn get(api_key: &String, id: &String, platform: &String) -> Result<Self, Error> {
        let url = format!(
            "https://api.mozambiquehe.re/bridge?auth={}&player={}&platform={}",
            api_key, id, platform
        );

        let url = Url::parse(&*url)?;
        let response = reqwest::get(url).await?;
 
        if !response.status().is_success() {
            return Err("I couldn't retrieve user information, maybe it doesn't exists?".into());
        }

        let res = response.json::<ApexPlayer>().await?;

        Ok(res)
    }
}

/// Fetch a player's information
#[poise::command(slash_command, prefix_command, track_edits)]
pub async fn player(
    ctx: Context<'_>,
    #[description = "Player id"] id: Option<String>,
    #[description = "Player platform"] platform: Option<String>,
) -> Result<(), Error> {
    let api_key = std::env::var("API_KEY").expect("No api key?");
    let response = ApexPlayer::get(&api_key, &id.unwrap(), &platform.unwrap()).await?;

    let status = match response.realtime.is_online {
    1 => "online",
    0 => "offline",
    _ => "offline",
};
    
    ctx.say(
        format!("{} is level {} on {}\nuser is actually {}", response.global.name, response.global.level, response.global.platform, status)
    ).await.unwrap();

    Ok(())
}


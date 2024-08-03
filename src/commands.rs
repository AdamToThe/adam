use poise::ReplyHandle;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{Context, Error};

#[derive(Debug, Serialize, Deserialize)]
struct YeQuote {
    quote: String
}

#[poise::command(
    prefix_command,
    track_edits, 
    slash_command,  
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel")]
pub async fn ping(
    ctx: Context<'_>
) -> Result<(), Error> {
    ctx.reply("Pong").await?;
    Ok(())

    
}


#[poise::command(
    prefix_command,
    track_edits, 
    slash_command,  
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel")]
pub async fn kanye(
    ctx: Context<'_>
) -> Result<(), Error> {

    let client = reqwest::Client::new();

    let res = client
        .get("https://api.kanye.rest/")
        .send()
        .await
        .unwrap();

    let q = match res.json::<YeQuote>().await {
        Ok(parsed) => parsed.quote,
        _ => "Not now, try later".into()
    };

    ctx.reply(q).await?;

    Ok(())

}
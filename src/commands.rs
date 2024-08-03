use serde::{Deserialize, Serialize};

use crate::{Context, ContextOP, Error};

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
    ctx.text("Pong").await?;
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

    ctx.text(&q).await?;

    Ok(())

}

// Helps I guess
#[poise::command(
    prefix_command,
    track_edits, 
    
    slash_command,  
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
    
)]
pub async fn help(ctx: Context<'_>, command: Option<String>) -> Result<(), Error> {
    let configuration = poise::builtins::PrettyHelpConfiguration {
        ephemeral: false,
        color: (73, 156, 160),
        show_context_menu_commands: true,
        include_description: true,
        extra_text_at_bottom: "kids....",
        ..Default::default()
    };
    poise::builtins::pretty_help(ctx, command.as_deref(), configuration).await?;
    Ok(())
}


// screenshots a webpage
#[poise::command(
    prefix_command,
    track_edits, 
    aliases("ss", "sc"),
    slash_command,  
    install_context = "Guild|User",
    interaction_context = "Guild|BotDm|PrivateChannel"
    
)]
pub async fn screenshot
        (ctx: Context<'_>, 
        #[description = "self-explanatory"] mut url: String
) -> Result<(), Error> {
    if !(url.starts_with("http://") || url.starts_with("https://")) {
        url = format!("https://{url}");
    }

    let ss = format!("https://image.thum.io/get/width/1080/crop/760/{url}");

    ctx.image(ss).await?;

    Ok(())
}
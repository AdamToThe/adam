use poise::{
    self,
    serenity_prelude::{
        self as serenity,
        model::gateway::GatewayIntents
    }
};
use tokio;
use dotenvy::dotenv;

mod commands;

use std::{env, sync::Arc, time::Duration};


type Error = Box<dyn std::error::Error + Send + Sync>;
#[allow(unused)]
type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {}

#[tokio::main]
async fn main() {
    dotenv().expect("No .env");

    let token = env::var("T").expect("No token");
    
    println!("Starting... ");

    let intents = GatewayIntents::all();

    let commands = vec![
        commands::ping(),
        commands::kanye()
    ];

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .options(poise::FrameworkOptions {
            commands: commands,
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("~".into()),
                edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                    Duration::from_secs(3600),
                ))),
                additional_prefixes: vec![
                    poise::Prefix::Literal("hey adam"),
                    poise::Prefix::Literal("hey adam,"),
                ],
                ..Default::default()
            },
            
            
            
            pre_command: |ctx: Context| {
                Box::pin(async move {
                    println!("Executing command {}...", ctx.command().qualified_name);
                })
            },
            
            post_command: |ctx| {
                Box::pin(async move {
                    println!("Executed command {}!", ctx.command().qualified_name);
                })
            },
            
            command_check: Some(|_ctx| {
                Box::pin(async move {
                    //if ctx.author().id == 123456789 {
                    //    return Ok(false);
                    //}
                    Ok(true)
                })
            }),
            
            skip_checks_for_owners: false,
            event_handler: |_ctx, event, _framework, _data| {
                Box::pin(event_handler(_ctx, event, _framework, _data))
            },
            ..Default::default()
        })
        .build();

    
    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;


    client.unwrap().start().await.unwrap();
    
}


async fn event_handler(
    _ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _: poise::FrameworkContext<'_, Data, Error>,
    _: &Data,
) -> Result<(), Error> {
    
    match event {
        serenity::FullEvent::Ready { 
            data_about_bot
        } => {
            println!("Logged in as {}", data_about_bot.user.name);
        },

        _ => {}
    }
    Ok(())
}
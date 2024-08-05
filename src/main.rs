use poise::{
    self, serenity_prelude::{
        self as serenity, model::gateway::GatewayIntents, Colour, CreateEmbed, CreateEmbedFooter, Error as SerenityError
    }, CreateReply, ReplyHandle
};
use tokio;
use dotenvy::dotenv;
use core::hash::Hash;

use markov::{Chain, Chainable};

mod commands;

use std::{env, fs::File, io::{BufRead, BufReader}, sync::Arc, time::Duration};


type Error = Box<dyn std::error::Error + Send + Sync>;
#[allow(unused)]
type Context<'a> = poise::Context<'a, Data, Error>;

trait ContextOP<'a> {
    async fn text(&self, text: &str) -> Result<ReplyHandle<'_>, SerenityError>;
    #[warn(dead_code)]
    async fn image(&self, url: String) -> Result<ReplyHandle<'_>, SerenityError>;
}

impl<'a> ContextOP<'a> for Context<'_> {
    async fn text(&self, text: &str) -> Result<ReplyHandle<'_>, SerenityError> {
        let embed = CreateEmbed::new()
            .color(Colour::from_rgb(43,45,49))
            .description(String::from(text))
            .footer(CreateEmbedFooter::new("[adam]"));

        let reply = CreateReply::default()
                .embed(embed);

        self.send( reply).await
    }

    #[warn(dead_code)]
    async fn image(&self, url: String) -> Result<ReplyHandle<'_>, SerenityError> {
        let embed = CreateEmbed::new()
            .color(Colour::from_rgb(43,45,49))
            .image(url);

        let reply = CreateReply::default()
                .embed(embed);

        self.send( reply).await
    }
}

pub struct Data {
    mark: Chain<String>
}

#[tokio::main]
async fn main() {
    dotenv().expect("No .env");

    let token = env::var("T").expect("No token");
    
    println!("Starting... ");

    let intents = GatewayIntents::all();

    let commands = vec![
        commands::ping(),
        commands::kanye(),
        commands::help(),
        commands::screenshot(),
        commands::ai(),
    ];

    
    let mut chain: Chain<String> = Chain::new();

    let file = File::open("messages.txt").expect("no messages.txt file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                chain.feed_str(&line);
            }
            _ => continue
        }
    }

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    mark: chain
                })
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
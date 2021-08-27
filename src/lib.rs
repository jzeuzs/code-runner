mod commands;

#[macro_use]
extern crate version;

use poise::serenity_prelude as serenity;
use serde::Deserialize;
use std::{
    collections::HashMap,
    env::var,
    time::Duration,
};

pub const EMBED_COLOR: serenity::Color = serenity::Color::from_rgb(47, 49, 54);

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type PrefixContext<'a> = poise::PrefixContext<'a, Data, Error>;

pub struct Data {
    http: reqwest::Client,
    owner_id: serenity::UserId,
}

pub async fn is_owner(ctx: PrefixContext<'_>) -> Result<bool, Error> {
    Ok(ctx.msg.author.id == ctx.data.owner_id)
}

async fn on_error(error: Error, ctx: poise::ErrorContext<'_, Data, Error>) {
    println!("Encountered error: {:?}", error);
    if let poise::ErrorContext::Command(ctx) = ctx {
        let reply = if let Some(poise::ArgumentParseError(error)) = error.downcast_ref() {
            if error.is::<poise::CodeBlockError>() {
                "Missing code block. Please use the following markdown:
\\`code here\\`
or
\\`\\`\\`language
code here
\\`\\`\\`"
                    .to_owned()
            } else {
                error.to_string()
            }
        } else {
            error.to_string()
        };
        if let Err(e) = poise::say_reply(ctx.ctx(), reply).await {
            println!("{}", e);
        }
    }
}

#[derive(Deserialize)]
struct Bin {
    key: String,
}

pub async fn post_bin(ctx: PrefixContext<'_>, content: String) -> Result<String, Error> {
    let json = ctx
        .data
        .http
        .post("https://paste.nomsy.net/documents")
        .header("Content-Type", "text/plain")
        .body(content)
        .send()
        .await?
        .json::<Bin>()
        .await?;
    let url = format!("https://paste.nomsy.net/{}", json.key);

    Ok(url)
}

async fn post_bot_list(guilds: usize) -> Result<(), Error> {
    let http = reqwest::Client::new();
    let mut body: HashMap<String, usize> = HashMap::new();

    body.insert("server_count".to_string(), guilds);

    http.post(format!("{}/post-bot-stats", var("API_URL")?))
        .json(&body)
        .header("Content-Type", "application/json")
        .send()
        .await?;

    Ok(())
}

async fn listener(
    ctx: &serenity::Context,
    event: &poise::Event<'_>,
    _framework: &poise::Framework<Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        poise::Event::Ready { data_about_bot } => {
            println!(
                "Bot is ready! Serving {} guilds",
                data_about_bot.guilds.len()
            );

            post_bot_list(data_about_bot.guilds.len()).await?;
            ctx.set_activity(serenity::Activity::playing("~run | ~help"))
                .await;

            Ok(())
        }
        _ => Ok(()),
    }
}

pub async fn main() -> Result<(), Error> {
    let mut options = poise::FrameworkOptions {
        prefix_options: poise::PrefixFrameworkOptions {
            additional_prefixes: &["run,", "run, ", "can you run, ", "run"],
            edit_tracker: Some(poise::EditTracker::for_timespan(Duration::from_secs(
                3600 * 24 * 2,
            ))),
            ..Default::default()
        },
        on_error: |error, ctx| Box::pin(on_error(error, ctx)),
        listener: |ctx, event, framework, data| Box::pin(listener(ctx, event, framework, data)),
        ..Default::default()
    };

    options.command(commands::help(), |f| f.category("Main"));
    options.command(commands::run(), |f| f.category("Main"));
    options.command(commands::source(), |f| f.category("Main"));
    options.command(commands::info(), |f| f.category("Main"));
    options.command(commands::vote(), |f| f.category("Main"));
    options.command(commands::yarn(), |f| f.category("Main"));
    options.command(commands::exec(), |f| f);

    let framework = poise::Framework::new(
        "~".to_owned(),
        serenity::ApplicationId(var("APPLICATION_ID")?.parse()?),
        move |_ctx, _bot, _framework| {
            Box::pin(async move {
                Ok(Data {
                    http: reqwest::Client::new(),
                    owner_id: serenity::UserId(566155739652030465),
                })
            })
        },
        options,
    );

    framework
        .start(serenity::ClientBuilder::new(&var("TOKEN")?))
        .await?;

    Ok(())
}

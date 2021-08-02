use poise::serenity_prelude as serenity;
use std::{env::var, time::Duration};

type Error = Box<dyn std::error::Error + Send + Sync>;
type PrefixContext<'a> = poise::PrefixContext<'a, Data, Error>;

struct Data {
    bot_user_id: serenity::UserId,
    http: reqwest::Client,
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
            } else if let poise::CommandErrorContext::Prefix(poise::PrefixCommandErrorContext {
                command:
                    poise::PrefixCommand {
                        options:
                            poise::PrefixCommandOptions {
                                multiline_help: Some(multiline_help),
                                ..
                            },
                        ..
                    },
                ..
            }) = ctx
            {
                format!("**{}**\n{}", error, multiline_help())
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

#[poise::command(track_edits, broadcast_typing)]
async fn hello(ctx: PrefixContext<'_>) -> Result<(), Error> {
    poise::say_reply(poise::Context::Prefix(ctx), format!("Hello, {}", ctx.msg.author)).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut options = poise::FrameworkOptions {
        prefix_options: poise::PrefixFrameworkOptions {
            additional_prefixes: &[
                "run,",
                "run, ",
                "can you run, "
            ],
            edit_tracker: Some(poise::EditTracker::for_timespan(
                Duration::from_secs(3600 * 24 * 2),
            )),
            ..Default::default()
        },
        on_error: |error, ctx| Box::pin(on_error(error, ctx)),
        ..Default::default()
    };

    options.command(hello(), |f| f.category("Main"));

    let framework = poise::Framework::new(
        "run".to_owned(),
        serenity::ApplicationId(var("APPLICATION_ID")?.parse()?),
        move |_ctx, bot, _framework| {
            Box::pin(async move {
                Ok(Data {
                    bot_user_id: bot.user.id,
                    http: reqwest::Client::new()
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

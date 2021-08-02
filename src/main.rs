use poise::serenity_prelude as serenity;
use serde::Deserialize;
use std::{env::var, time::Duration};

type Error = Box<dyn std::error::Error + Send + Sync>;
type PrefixContext<'a> = poise::PrefixContext<'a, Data, Error>;

struct Data {
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
struct Response {
    language: String,
    output: String,
    stderr: String,
}

#[derive(Deserialize)]
struct Bin {
    key: String,
}

async fn execute_piston(
    ctx: PrefixContext<'_>,
    lang: String,
    code: String,
) -> Result<Response, Error> {
    let params = [("lang", lang), ("code", code)];
    let json = ctx
        .data
        .http
        .post(var("API_URL")?)
        .form(&params)
        .send()
        .await?
        .json::<Response>()
        .await?;

    Ok(json)
}

async fn post_bin(ctx: PrefixContext<'_>, content: String) -> Result<String, Error> {
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

#[poise::command(track_edits, broadcast_typing)]
async fn hello(ctx: PrefixContext<'_>) -> Result<(), Error> {
    poise::say_reply(
        poise::Context::Prefix(ctx),
        format!("Hello, {}", ctx.msg.author),
    )
    .await?;

    Ok(())
}

#[poise::command(track_edits)]
async fn help(
    ctx: PrefixContext<'_>,
    #[description = "A command to show help for."] command: Option<String>,
) -> Result<(), Error> {
    let bottom_text = "Type run help command for more info on a command.
You can edit your message to the bot and the bot will edit its response.";

    poise::defaults::help(
        poise::Context::Prefix(ctx),
        command.as_deref(),
        bottom_text,
        poise::defaults::HelpResponseMode::Default,
    )
    .await?;

    Ok(())
}

#[poise::command(track_edits, broadcast_typing)]
async fn run(ctx: PrefixContext<'_>, code: poise::CodeBlock) -> Result<(), Error> {
    match code.language {
        None => {
            poise::say_reply(
                poise::Context::Prefix(ctx),
                "The codeblock is missing a language...".to_string(),
            )
            .await?;

            Ok(())
        }
        Some(lang) => {
            let res = execute_piston(ctx, lang, code.code).await;

            match res {
                Err(e) => {
                    poise::say_reply(
                        poise::Context::Prefix(ctx),
                        "You provided an invalid language...".to_string(),
                    )
                    .await?;

                    println!("{:?}", e);

                    Ok(())
                }
                Ok(re) => {
                    if re.output.chars().count() > 500 {
                        let url = post_bin(ctx, re.output).await?;
                        let msg = format!("The response was longer than 500 characters, so I uploaded it to a paste!
<{}>
                        ", url);

                        poise::say_reply(poise::Context::Prefix(ctx), msg).await?;
                        Ok(())
                    } else if re.stderr.chars().count() > 0 {
                        let msg = format!(
                            "An error occured!
```sh
{}
```
                        ",
                            re.stderr
                        );

                        poise::say_reply(poise::Context::Prefix(ctx), msg).await?;
                        Ok(())
                    } else {
                        let lang = re.language;
                        let msg = format!(
                            "```{}
{}
```
                        ",
                            &lang, re.output
                        );

                        poise::say_reply(poise::Context::Prefix(ctx), msg).await?;
                        Ok(())
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut options = poise::FrameworkOptions {
        prefix_options: poise::PrefixFrameworkOptions {
            additional_prefixes: &["run,", "run, ", "can you run, ", "run"],
            edit_tracker: Some(poise::EditTracker::for_timespan(Duration::from_secs(
                3600 * 24 * 2,
            ))),
            ..Default::default()
        },
        on_error: |error, ctx| Box::pin(on_error(error, ctx)),
        ..Default::default()
    };

    options.command(hello(), |f| f.category("Main"));
    options.command(help(), |f| f.category("Main"));
    options.command(run(), |f| f.category("Main"));

    let framework = poise::Framework::new(
        "run ".to_owned(),
        serenity::ApplicationId(var("APPLICATION_ID")?.parse()?),
        move |_ctx, _bot, _framework| {
            Box::pin(async move {
                Ok(Data {
                    http: reqwest::Client::new(),
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

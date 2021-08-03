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

#[poise::command(track_edits, explanation_fn = "help_help")]
async fn help(ctx: PrefixContext<'_>, command: Option<String>) -> Result<(), Error> {
    let bottom_text = "Type ~help command for more info on a command.
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

fn help_help() -> String {
    "Provides a list of all commands.
Or if a command is provided, it will show information about that specific command.
    "
    .to_string()
}

#[poise::command(track_edits, broadcast_typing, explanation_fn = "run_help")]
async fn run(ctx: PrefixContext<'_>, code: poise::CodeBlock) -> Result<(), Error> {
    match code.language {
        None => {
            poise::say_reply(
                poise::Context::Prefix(ctx),
                "The codeblock is missing a language...
Visit https://github.com/1chiSensei/code-runner#supportedlanguages to know all of the supported languages!
                ".to_string(),
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
                        "You provided an invalid language...
Visit https://github.com/1chiSensei/code-runner#supportedlanguages to know all of the supported languages!
                        ".to_string(),
                    )
                    .await?;

                    println!("{:?}", e);

                    Ok(())
                }
                Ok(re) => {
                    if re.output.chars().count() > 1000 {
                        let url = post_bin(ctx, re.output).await?;
                        let msg = format!("The response was longer than 1k characters, so I uploaded it to a paste!
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

fn run_help() -> String {
    "Executes code in a virtual sandbox.
If the result is more than 1k characters, it will post the result to a paste.
If an error occurred, it will send the error.
    "
    .to_string()
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

    options.command(help(), |f| f.category("Main"));
    options.command(run(), |f| f.category("Main"));

    let framework = poise::Framework::new(
        "~".to_owned(),
        serenity::ApplicationId(var("APPLICATION_ID")?.parse()?),
        move |ctx, _bot, _framework| {
            Box::pin(async move {
                ctx.set_activity(serenity::Activity::playing("")).await;

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

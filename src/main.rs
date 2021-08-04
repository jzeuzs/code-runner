#[macro_use]
extern crate version;

use poise::serenity_prelude as serenity;
use serde::Deserialize;
use std::{env::var, process::Command, time::Duration};
use sys_info::linux_os_release;

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

#[poise::command(
    track_edits,
    broadcast_typing,
    explanation_fn = "run_help",
    aliases("r")
)]
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

#[poise::command(track_edits, explanation_fn = "source_help", aliases("github"))]
async fn source(ctx: PrefixContext<'_>) -> Result<(), Error> {
    let msg = "This is where my code is publicly hosted!
https://github.com/1chiSensei/code-runner
    ";

    poise::say_prefix_reply(ctx, msg.to_string()).await?;
    Ok(())
}

fn source_help() -> String {
    "Shows the github repository of the bot.".to_string()
}

#[poise::command(track_edits, explanation_fn = "info_help", aliases("information"))]
async fn info(ctx: PrefixContext<'_>) -> Result<(), Error> {
    let rust_ver = Command::new("rustc").arg("--version").output()?;
    let cargo_ver = Command::new("cargo").arg("--version").output()?;
    let os = linux_os_release()?;
    let msg = format!(
        "I am a bot that runs code.
Supported Languages: <https://github.com/1chiSensei/code-runner#supportedlanguages>

Version: `{}`
Rust: `{}`
Cargo: `{}`
OS: `{}`
    ",
        version!(),
        String::from_utf8(rust_ver.stdout)?,
        String::from_utf8(cargo_ver.stdout)?,
        os.pretty_name.unwrap_or("Linux".to_string())
    );

    poise::say_prefix_reply(ctx, msg).await?;
    Ok(())
}

fn info_help() -> String {
    "Shows general information about the bot.".to_string()
}

async fn listener(
    ctx: &serenity::Context,
    event: &poise::Event<'_>,
    _framework: &poise::Framework<Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        poise::Event::Ready { data_about_bot: _ } => {
            ctx.set_activity(serenity::Activity::playing("~run | ~help"))
                .await;

            Ok(())
        }
        _ => Ok(()),
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
        listener: |ctx, event, framework, data| Box::pin(listener(ctx, event, framework, data)),
        ..Default::default()
    };

    options.command(help(), |f| f.category("Main"));
    options.command(run(), |f| f.category("Main"));
    options.command(source(), |f| f.category("Main"));
    options.command(info(), |f| f.category("Main"));

    let framework = poise::Framework::new(
        "~".to_owned(),
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

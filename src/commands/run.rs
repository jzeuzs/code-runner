use crate::{post_bin, Error, PrefixContext};
use serde::Deserialize;
use std::env::var;

#[derive(Deserialize)]
struct Response {
    language: String,
    output: String,
    stderr: String,
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

#[poise::command(
    prefix_command,
    track_edits,
    broadcast_typing,
    explanation_fn = "run_help",
    aliases("r")
)]
pub async fn run(ctx: PrefixContext<'_>, code: poise::CodeBlock) -> Result<(), Error> {
    match code.language {
        None => {
            poise::say_reply(
                poise::Context::Prefix(ctx),
                "The codeblock is missing a language...
Visit https://github.com/1chiSensei/code-runner#supported-languages to know all of the supported languages!
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
Visit https://github.com/1chiSensei/code-runner#supported-languages to know all of the supported languages!
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

                        poise::send_prefix_reply(ctx, |m| m.content(msg)).await?;
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

                        poise::send_prefix_reply(ctx, |m| m.content(msg)).await?;
                        Ok(())
                    } else if re.output.chars().count() == 0 {
                        poise::send_prefix_reply(ctx, |m| m.content("Your code yielded no results.")).await?;
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

                        poise::send_prefix_reply(ctx, |m| m.content(msg)).await?;
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

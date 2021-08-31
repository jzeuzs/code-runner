use crate::{Error, PrefixContext, EMBED_COLOR};
use serde::Deserialize;
use std::env::var;

#[derive(Deserialize)]
struct Response {
    name: String,
    url: String,
    description: String,
}

async fn fetch_yarn(ctx: PrefixContext<'_>, name: String) -> Result<Response, Error> {
    let data = ctx
        .data
        .http
        .get(format!("{}/yarn", var("API_URL")?))
        .query(&[("name", name)])
        .send()
        .await?
        .json::<Response>()
        .await?;

    Ok(data)
}

#[poise::command(
    track_edits,
    broadcast_typing,
    explanation_fn = "yarn_help",
    aliases("npm", "pnpm", "node-pkg")
)]
pub async fn yarn(ctx: PrefixContext<'_>, name: String) -> Result<(), Error> {
    let d = fetch_yarn(ctx, name).await;

    match d {
        Err(_) => {
            poise::say_prefix_reply(ctx, "That package could not be found.".to_string()).await?;
            Ok(())
        }
        Ok(data) => {
            poise::send_prefix_reply(ctx, |m| {
                m.embed(|m| {
                    m.title(data.name)
                        .url(data.url)
                        .description(data.description)
                        .color(EMBED_COLOR)
                })
            })
            .await?;
            Ok(())
        }
    }
}

fn yarn_help() -> String {
    "Gives information about a NPM/Yarn package".to_string()
}

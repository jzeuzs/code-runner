use crate::{Error, PrefixContext, EMBED_COLOR};
use serde::Deserialize;
use std::env::var;

#[derive(Deserialize)]
struct Response {
    name: String,
    url: String,
    description: String,
}

#[poise::command(
    track_edits,
    broadcast_typing,
    explanation_fn = "yarn_help",
    aliases("npm", "pnpm", "node-pkg")
)]
pub async fn yarn(ctx: PrefixContext<'_>, name: String) -> Result<(), Error> {
    let data = ctx
        .data
        .http
        .get(format!("{}/npm", var("API_URL")?))
        .query(&[("name", name)])
        .send()
        .await?
        .json::<Response>()
        .await?;

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

fn yarn_help() -> String {
    "Gives information about a NPM/Yarn package".to_string()
}

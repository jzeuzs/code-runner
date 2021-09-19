use crate::{Error, PrefixContext, EMBED_COLOR};
use serde::Deserialize;
use std::env::var;

#[derive(Deserialize)]
struct Response {
    url: String,
}

#[poise::command(prefix_command, track_edits, broadcast_typing, explanation_fn = "format_help")]
pub async fn format(ctx: PrefixContext<'_>, code: poise::CodeBlock) -> Result<(), Error> {
    let img = ctx
        .data
        .http
        .post(format!("{}/format", var("API_URL")?))
        .form(&[("code", code.code)])
        .send()
        .await?
        .json::<Response>()
        .await?;

    poise::send_prefix_reply(ctx, |m| m.embed(|m| m.image(img.url).color(EMBED_COLOR))).await?;
    Ok(())
}

fn format_help() -> String {
    "Formats your code into an image.".to_string()
}

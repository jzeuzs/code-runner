use crate::{Error, PrefixContext};

#[poise::command(prefix_command, track_edits, explanation_fn = "source_help", aliases("github"))]
pub async fn source(ctx: PrefixContext<'_>) -> Result<(), Error> {
    let msg = "This is where my code is publicly hosted!
https://github.com/1chiSensei/code-runner
    ";

    poise::send_prefix_reply(ctx, |m| m.content(msg)).await?;
    Ok(())
}

fn source_help() -> String {
    "Shows the github repository of the bot.".to_string()
}

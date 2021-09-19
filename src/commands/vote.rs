use crate::{Error, PrefixContext};

#[poise::command(prefix_command, track_edits, explanation_fn = "vote_help")]
pub async fn vote(ctx: PrefixContext<'_>) -> Result<(), Error> {
    let msg = "Vote for me here!
<https://top.gg/bot/871593892280160276/vote>
<https://infinitybotlist.com/bots/871593892280160276/vote>
    ";

    poise::send_prefix_reply(ctx, |m| m.content(msg)).await?;
    Ok(())
}

fn vote_help() -> String {
    "Shows where you can vote for the bot.".to_string()
}

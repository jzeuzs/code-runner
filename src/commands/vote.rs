use crate::{Error, PrefixContext};

#[poise::command(track_edits, explanation_fn = "vote_help")]
pub async fn vote(ctx: PrefixContext<'_>) -> Result<(), Error> {
    let msg = "Vote for me here!
<https://top.gg/bot/871593892280160276/vote>
<https://infinitybotlist.com/bots/871593892280160276/vote>
    ";

    poise::say_prefix_reply(ctx, msg.to_string()).await?;
    Ok(())
}

fn vote_help() -> String {
    "Shows where you can vote for the bot.".to_string()
}

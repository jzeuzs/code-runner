use crate::{Error, PrefixContext};

#[poise::command(prefix_command, track_edits, explanation_fn = "help_help")]
pub async fn help(ctx: PrefixContext<'_>, command: Option<String>) -> Result<(), Error> {
    let bottom_text = "Type ~help command for more info on a command.
You can edit your message to the bot and the bot will edit its response.";

    poise::samples::help(
        poise::Context::Prefix(ctx),
        command.as_deref(),
        bottom_text,
        poise::samples::HelpResponseMode::Default,
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

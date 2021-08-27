use crate::{Error, PrefixContext};
use std::process::Command;
use sys_info::linux_os_release;

#[poise::command(track_edits, explanation_fn = "info_help", aliases("information"))]
pub async fn info(ctx: PrefixContext<'_>) -> Result<(), Error> {
    let rust_ver = Command::new("rustc").arg("--version").output()?;
    let cargo_ver = Command::new("cargo").arg("--version").output()?;
    let os = linux_os_release()?;
    let msg = format!(
        "I am a bot that runs code.
Supported Languages: <https://github.com/1chiSensei/code-runner#supported-languages>

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

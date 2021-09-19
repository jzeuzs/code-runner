use crate::{Error, PrefixContext};
use std::{process::Command, fs::File, io::Read};

fn parse_line(l: String) -> Option<(String, String)> {
    let words: Vec<&str> = l.splitn(2, "=").collect();

    if words.len() < 2 {
        return None;
    }

    let mut trim_value = String::from(words[1]);

    if trim_value.starts_with("\"") {
        trim_value.remove(0);
    }

    if trim_value.ends_with("\"") {
        let len = trim_value.len();

        trim_value.remove(len - 1);
    }

    Some((String::from(words[0]), trim_value))
}

fn get_os_name() -> Result<String, Error> {
    let mut s = String::new();

    File::open("/etc/os-release")?.read_to_string(&mut s)?;

    let mut name = "Linux".to_string();

    for l in s.split("\n") {
        match parse_line(l.trim().to_string()) {
            Some((key, value)) => match (key.as_ref(), value) {
                ("PRETTY_NAME", val) => name = val,
                _ => {}
            }
            None => {}
        }
    }

    Ok(name)
}

#[poise::command(prefix_command, track_edits, explanation_fn = "info_help", aliases("information"))]
pub async fn info(ctx: PrefixContext<'_>) -> Result<(), Error> {
    let rust_ver = Command::new("rustc").arg("--version").output()?;
    let cargo_ver = Command::new("cargo").arg("--version").output()?;
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
        get_os_name()?
    );

    poise::send_prefix_reply(ctx, |m| m.content(msg)).await?;
    Ok(())
}

fn info_help() -> String {
    "Shows general information about the bot.".to_string()
}

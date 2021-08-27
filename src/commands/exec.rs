use crate::{Error, PrefixContext, is_owner, post_bin};
use std::process::Stdio;
use execute::{command, Execute};

#[poise::command(track_edits, check = "is_owner", hide_in_help, aliases("sh", "bash", "$"))]
pub async fn exec(ctx: PrefixContext<'_>, #[rest] code: String) -> Result<(), Error> {
    let mut command = command(code);

    command.stdout(Stdio::piped());

    let output = String::from_utf8(command.execute_output()?.stdout)?;

    if output.chars().count() > 1500 {
        let url = post_bin(ctx, output).await?;
        let msg = format!("<{}>", url);

        poise::say_prefix_reply(ctx, msg).await?;
        Ok(())
    } else {
        let msg = format!(
            "```sh
{}
```
        ",
            output
        );

        poise::say_prefix_reply(ctx, msg).await?;
        Ok(())
    }
}

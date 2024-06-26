use crate::{Context, Error};

/// Show this menu
#[poise::command(slash_command, prefix_command, track_edits)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"] command: Option<String>,
) -> Result<(), Error> {
    let config = poise::builtins::HelpConfiguration {
        extra_text_at_bottom: "Octane Discord Bot.",
        ..Default::default()
    };
    poise::builtins::help(ctx, command.as_deref(), config).await?;
    Ok(())
}

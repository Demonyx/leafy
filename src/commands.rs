#![feature(iter_intersperse)]

use crate::{markov, Context, Error};
use poise::serenity_prelude as serenity;

/// Displays your or another user's account creation date
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn names(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let name_list = markov::generate(5);
    let mut response = String::new();
    response.push_str("Vote for the name for your plant: \n");
    // let reacts = ['🌲', '🌱', '🌵', '🍀', '🌿'];
    let reacts = ['🌻', '🌹', '🌷', '💐', '🌸'];

    let mut i = 0;
    for name in name_list {
        response.push_str(&format!("{} ", reacts[i]));
        response.push_str(&name);
        response.push_str("\n");
        i += 1;
    }

    let reply_handle = ctx.send(|message| message.content(response)).await?;
    let message = reply_handle.message().await?;
    for emoji in reacts {
        message.react(ctx.serenity_context(), emoji).await?;
    }

    Ok(())
}

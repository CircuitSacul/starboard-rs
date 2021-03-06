use std::sync::Arc;

use twilight_model::application::interaction::Interaction;

use crate::client::bot::StarboardBot;

use super::{
    autocomplete::handle::handle_autocomplete, commands::handle::handle_command,
    components::handle::handle_component,
};

pub async fn handle_interaction(
    shard_id: u64,
    interaction: Interaction,
    bot: Arc<StarboardBot>,
) -> anyhow::Result<()> {
    match interaction {
        Interaction::ApplicationCommand(interaction) => {
            handle_command(shard_id, bot, interaction).await?
        }
        Interaction::ApplicationCommandAutocomplete(interaction) => {
            handle_autocomplete(bot, interaction).await?
        }
        Interaction::MessageComponent(interaction) => handle_component(bot, interaction).await?,
        _ => {}
    }

    Ok(())
}

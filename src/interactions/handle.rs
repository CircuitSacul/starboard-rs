use std::sync::Arc;

use twilight_model::application::interaction::Interaction;

use crate::client::bot::Starboard;
use crate::interactions::commands::handle::handle_command;

pub async fn handle_interaction(shard_id: u64, interaction: Interaction, bot: Arc<Starboard>) {
    match interaction {
        Interaction::ApplicationCommand(command) => {
            handle_command(shard_id, bot, command).await;
        }
        _ => {}
    }
}

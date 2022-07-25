use twilight_model::gateway::payload::incoming::{
    ReactionAdd, ReactionRemove, ReactionRemoveAll, ReactionRemoveEmoji,
};

use crate::{
    client::bot::StarboardBot,
    core::emoji::SimpleEmoji,
    database::{Member, Message, User, Vote},
    map_dup_none, unwrap_id,
};

use super::{config::StarboardConfig, vote_status::VoteStatus};

pub async fn handle_reaction_add(
    bot: &StarboardBot,
    event: Box<ReactionAdd>,
) -> anyhow::Result<()> {
    let guild_id = match event.guild_id {
        None => return Ok(()),
        Some(guild_id) => guild_id,
    };
    if !bot.cache.guilds.contains_key(&guild_id) {
        return Ok(());
    }
    let reactor_member = event
        .member
        .as_ref()
        .expect("No member object in reaction_add");
    if reactor_member.user.bot {
        return Ok(());
    }

    let orig_msg = Message::get_original(&bot.pool, unwrap_id!(event.message_id)).await?;
    let orig_msg = match orig_msg {
        None => {
            // author data (todo)
            todo!();

            // message
            let orig = map_dup_none!(Message::create(
                &bot.pool,
                unwrap_id!(event.message_id),
                unwrap_id!(guild_id),
                unwrap_id!(event.channel_id),
                0,
                false,
            ))?;

            match orig {
                Some(msg) => msg,
                None => Message::get_original(&bot.pool, unwrap_id!(event.message_id))
                    .await?
                    .unwrap(),
            }
        }
        Some(msg) => msg,
    };

    let emoji = SimpleEmoji::from(event.emoji.clone());
    let configs = StarboardConfig::list_for_channel(bot, guild_id, event.channel_id).await?;
    let status =
        VoteStatus::get_vote_status(bot, &emoji, &configs, event.message_id, event.channel_id)
            .await;

    match status {
        VoteStatus::Ignore => Ok(()),
        VoteStatus::Remove => {
            let _ = bot
                .http
                .delete_reaction(
                    event.channel_id,
                    event.message_id,
                    &emoji.reactable(),
                    event.user_id,
                )
                .exec()
                .await;

            Ok(())
        }
        VoteStatus::Valid((upvote, downvote)) => {
            // create reactor data
            let reactor_user_id = unwrap_id!(reactor_member.user.id);
            map_dup_none!(User::create(
                &bot.pool,
                reactor_user_id,
                reactor_member.user.bot
            ))?;
            map_dup_none!(Member::create(
                &bot.pool,
                reactor_user_id,
                unwrap_id!(guild_id)
            ))?;

            for config in upvote {
                Vote::create(
                    &bot.pool,
                    orig_msg.message_id,
                    config.starboard.id,
                    reactor_user_id,
                    orig_msg.author_id,
                    false,
                ).await?;
            }
            for config in downvote {
                Vote::create(
                    &bot.pool,
                    orig_msg.message_id,
                    config.starboard.id,
                    reactor_user_id,
                    orig_msg.author_id,
                    true,
                ).await?;
            }

            Ok(())
        }
    }
}

pub async fn handle_reaction_remove(
    _bot: &StarboardBot,
    _event: Box<ReactionRemove>,
) -> anyhow::Result<()> {
    todo!()
}

pub async fn handle_reaction_remove_all(
    _bot: &StarboardBot,
    _event: ReactionRemoveAll,
) -> anyhow::Result<()> {
    todo!()
}

pub async fn handle_reaction_remove_emoji(
    _bot: &StarboardBot,
    _event: ReactionRemoveEmoji,
) -> anyhow::Result<()> {
    todo!()
}
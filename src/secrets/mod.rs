use crate::utils::constants;
use anyhow::Context;
use std::fs;

/// Attempt to gather the Discord token.
pub(crate) fn get_discord_token() -> anyhow::Result<String> {
    fs::read_to_string(constants::DISCORD_TOKEN_SECRET)
        .context(format!("{} not found", constants::DISCORD_TOKEN_SECRET))
}

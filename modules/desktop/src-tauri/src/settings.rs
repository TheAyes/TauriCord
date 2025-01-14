pub enum DiscordVersion {
    Stable,
    Canary,
    PTB,
}

pub struct Settings {
    pub discord_version: DiscordVersion,
}

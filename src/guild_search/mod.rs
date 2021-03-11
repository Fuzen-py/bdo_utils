pub struct Guild {
    pub name: String,
    pub token: GuildToken,
}
pub struct GuildToken(pub(crate) String);

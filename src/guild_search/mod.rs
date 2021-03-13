pub struct GuildQuery {
    pub name: String,
    pub token: GuildToken,
}

pub struct Guild {}

pub struct GuildToken(pub(crate) String);

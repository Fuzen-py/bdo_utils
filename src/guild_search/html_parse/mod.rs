use scraper::{ElementRef, Html, Selector};

use crate::{
    guild_search::{Guild, GuildQuery, GuildToken, PlayerCache},
    models::Region,
    player_search::PlayerProfile,
};

use super::GuildPreview;

pub(crate) fn prase_search_page(body: &str) -> Vec<GuildPreview> {
    let document: Html = Html::parse_document(&body);
    let guild_list_selector = Selector::parse(
        "#wrap > div > div.container.guild > article > div > div > div.box_list_area > ul > li",
    )
    .unwrap();
    unimplemented!()
}

use chrono::{Date, DateTime};
use scraper::{selector, ElementRef, Html, Selector};

use crate::{
    guild_search::{GuildQuery, GuildToken},
    models::Region,
    PlayerToken,
};

use super::search_result::{GuildCache, PlayerResult, Profile};

pub(crate) fn prase_search_page(body: &str) -> Vec<PlayerResult> {
    let document = Html::parse_document(body);
    let adventure_list_selector = Selector::parse(".adventure_list_table > li").unwrap();
    let adventures = document
        .select(&adventure_list_selector)
        .filter_map(parse_adventurer_li)
        .collect();
    adventures
}

fn parse_adventurer_li(elem: ElementRef) -> Option<PlayerResult> {
    let region_sel = Selector::parse("span.region_info").ok()?;
    let family_name_sel = Selector::parse("div.title > a").ok()?;
    let main_level_sel =
        Selector::parse("span.character_desc > span.text_area > span.level").ok()?;
    let main_charname_sel =
        Selector::parse("span.character_desc > span.text_area > span.text").ok()?;
    let main_class_sel = Selector::parse("span.character_class > span.name").ok()?;
    let guild_sel = Selector::parse("div.state > a").ok()?;
    let region = elem.select(&region_sel).next()?.text().next()?;
    let main_level = elem.select(&main_level_sel).next()?.text().next()?;
    let main_charname = elem.select(&main_charname_sel).next()?.text().next()?;
    let main_class = elem
        .select(&main_class_sel)
        .next()?
        .text()
        .next()?
        .trim()
        .to_owned();
    let guild_elem = elem.select(&guild_sel).next()?;
    let family_elem = elem.select(&family_name_sel).next()?;
    let family = family_elem.text().next().map(|s| s.trim())?.to_owned();
    let token = family_elem.value().attr("href")?.trim().to_owned();
    let region = match region {
        "NA" => Region::NA,
        "EU" => Region::EU,
        _ => unimplemented!("Unsupported Region"),
    };
    let level = {
        let lvl_txt = main_level.trim().strip_prefix("Lv.")?;
        if lvl_txt.is_empty() || lvl_txt.eq_ignore_ascii_case("private") {
            None
        } else {
            Some(lvl_txt.parse().ok()?)
        }
    };

    let player = PlayerResult {
        level,
        region,
        family,
        token: super::PlayerToken(token),
        class: main_class,
        // TODO: Parse Guild
        guild: None,
    };

    Some(player)
}

fn parse_profile_page(body: &str, token: PlayerToken) -> Option<Profile> {
    let document = Html::parse_document(body);
    let family_name_sel = Selector::parse("p.nick").ok()?;
    let _family = document
        .select(&family_name_sel)
        .next()
        .and_then(|e| e.text().next())
        .map(|s| s.trim().to_owned())?;
    let guild_sel = Selector::parse("span.desc.guild > a").ok()?;
    let guild_private_sel = Selector::parse("span.desc.guild > span").ok()?;
    let _guild = {
        if document.select(&guild_private_sel).next().is_some() {
            None
        } else {
            let guild_elm = document.select(&guild_sel).next()?;
            let name = guild_elm.text().next()?.trim().to_owned();
            let token = guild_elm.value().attr("href")?.to_owned();
            Some(GuildQuery {
                name,
                token: GuildToken(token),
            })
        }
    };

    Profile {
        guild: None,
        crated: unimplemented!(),
        characters: unimplemented!(),
    };

    None
}

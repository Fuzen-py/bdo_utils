use scraper::{ElementRef, Html, Selector};

use crate::{
    guild_search::{GuildQuery, GuildToken},
    models::{
        lifeskill_level::{Grade, Level, LifeSkillLevel},
        Region,
    },
    PlayerToken,
};

use super::search_result::{Character, CharacterLifeskills, GuildCache, PlayerResult, Profile};

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

pub(crate) fn parse_profile_page(
    body: &str,
    token: PlayerToken,
    region: Region,
) -> Option<Profile> {
    let document = Html::parse_document(body);
    let family_name_sel = Selector::parse("p.nick").ok()?;
    let family = document
        .select(&family_name_sel)
        .next()
        .and_then(|e| e.text().next())
        .map(|s| s.trim().to_owned())?;

    let guild = {
        let guild_sel = Selector::parse("span.desc.guild > a").ok()?;
        let guild_private_sel = Selector::parse("span.desc.guild > span").ok()?;

        if document.select(&guild_private_sel).next().is_some() {
            None
        } else {
            let guild_elm = document.select(&guild_sel).next()?;
            let name = guild_elm.text().next()?.trim().to_owned();
            let token = guild_elm.value().attr("href")?.to_owned();
            let query = GuildQuery {
                name,
                token: GuildToken(token),
            };
            Some(GuildCache::Unprocessed(query))
        }
    };

    let characters = {
        let char_sel =
            Selector::parse("ul.character_list > li").expect("Failed to get CSS Selector");
        document
            .select(&char_sel)
            .filter_map(parse_character)
            .collect()
    };

    let created = {
        let selector = Selector::parse("li:nth-child(2) > span.desc").unwrap();
        let s = document
            .select(&selector)
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap()
            .trim();

        chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
    };

    Some(Profile {
        name: family,
        guild,
        characters,
        created,
        token,
        region,
    })
}

fn parse_character(elem: ElementRef) -> Option<Character> {
    let name = {
        let sel = Selector::parse("p.character_name").unwrap();
        elem.select(&sel).next()?.text().next()?.trim().to_owned()
    };
    let class = {
        let selector =
            Selector::parse("p.character_info > span.character_symbol > em:nth-child(2)").unwrap();
        elem.select(&selector)
            .next()?
            .text()
            .next()?
            .trim()
            .to_owned()
    };

    let contribution = {
        let selector = Selector::parse("p.character_info > span:nth-child(3) > em").unwrap();
        let txt = elem.select(&selector).next()?.text().next()?.trim();
        if txt.eq_ignore_ascii_case("private") {
            None
        } else {
            Some(txt.parse().ok()?)
        }
    };

    let level = {
        let selector = Selector::parse("p.character_info > span:nth-child(2) > em").unwrap();
        let txt = elem.select(&selector).next()?.text().next()?.trim();
        if txt.eq_ignore_ascii_case("private") {
            None
        } else {
            Some(txt.parse().ok()?)
        }
    };

    let lifeskills = {
        let selector = Selector::parse("div.character_spec > ul").unwrap();
        if let Some(lifeskill_ul) = elem.select(&selector).next() {
            let mut lifeskills = CharacterLifeskills::default();
            let li_sel = Selector::parse("li").unwrap();
            // TODO: Go off of lifeskill's icon class not position
            for (pos, li) in lifeskill_ul.select(&li_sel).enumerate() {
                match pos {
                    0 => lifeskills.gathering = parse_lifeskills(li),
                    1 => lifeskills.fishing = parse_lifeskills(li),
                    2 => lifeskills.hunting = parse_lifeskills(li),
                    3 => lifeskills.cooking = parse_lifeskills(li),
                    4 => lifeskills.alchemy = parse_lifeskills(li),
                    5 => lifeskills.processing = parse_lifeskills(li),
                    6 => lifeskills.training = parse_lifeskills(li),
                    7 => lifeskills.trading = parse_lifeskills(li),
                    8 => lifeskills.farming = parse_lifeskills(li),
                    9 => lifeskills.sailing = parse_lifeskills(li),
                    10 => lifeskills.bartering = parse_lifeskills(li),
                    _ => unimplemented!(),
                }
            }
            unimplemented!()
        } else {
            // Lifeskills is private
            None
        }
    };

    let is_main: bool = {
        let selector = Selector::parse("p.character_name > span.selected_label").unwrap();
        elem.select(&selector)
            .any(|n| n.text().any(|s| s.eq_ignore_ascii_case("Main Character")))
    };

    Some(Character {
        name,
        class,
        contribution,
        level,
        lifeskills,
        is_main,
    })
}

fn parse_lifeskills(li: ElementRef) -> LifeSkillLevel {
    let selector = Selector::parse("span.spec_Level").unwrap();
    let mut txt = li.select(&selector).next().map(|e| e.text()).unwrap();
    let grade: Grade = {
        txt.next()
            .unwrap()
            .trim()
            .to_ascii_lowercase()
            .parse()
            .unwrap()
    };
    let level = {
        let level = txt.next().unwrap().trim().parse().unwrap();
        Level {
            level,
            progress: 0.0,
        }
    };
    LifeSkillLevel { grade, level }
}

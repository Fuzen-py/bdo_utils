mod item;
mod locations;
use item::ItemWeight;
//use serde_json::Value as JsonValue;
mod html_parse;

pub use item::{Item, ItemInfo, ItemName, ItemPrice};
pub use locations::Location;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct SearchQuery {
    l: &'static str,
    term: String,
}

/*
// Search for an Item
// TODO: Figure out how to make sure an item is.. an item
async fn search_item(term: String) -> Result<Vec<SearchResult>, surf::Error> {
    let query = SearchQuery {
        l: "us",
        term
    };
    let mut resp = dbg!(surf::get("https://bdocodex.com/ac.php")
        .query(&query)?)
        .await?;

    let body = dbg!(resp.body_string().await?);
    let json: JsonValue = serde_json::from_str(&body)?;
    let results: Vec<SearchResult> = json.into_iter().filter_map(parse_search_results).collect();

    Ok(results)
}

async fn get_item_info(id: u128) -> Result<Option<ItemInfo>, surf::Error> {
    let uri = format!(
        "https://bdocodex.com/tip.php?id=item--{}&quest_group=&l=us&nf=on",
        id
    );
    let item: Item = Item(id);
    let html_body = surf::get(uri).await?.body_string().await?;
    if let Some(tooltip) = html_parse::parse_tooltip(html_body) {
        Ok(Some(ItemInfo {
            item,
            name: ItemName {
                language: String::from("en-us"),
                item,
                name: tooltip.name,
            },
            price: ItemPrice {
                item,
                buy: tooltip.buy,
                sell: tooltip.sell,
            },
            weight: ItemWeight {
                item,
                weight: tooltip.weight,
            },
        }))
    } else {
        Ok(None)
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct SearchResult {
    value: String,
    name: String,
    grade: String,
    link: String,
}

// Removes any results that do not fit the Item schema
fn parse_search_results(v: JsonValue) -> Option<SearchResult> {
    serde_json::from_value(v).ok()
}

*/
// curl "https://bdocodex.com/ac.php?l=us^&term=Cron+Stone" ^
//  -H "authority: bdocodex.com" ^
//  -H "pragma: no-cache" ^
//  -H "cache-control: no-cache" ^
//  -H "dnt: 1" ^
//  -H "upgrade-insecure-requests: 1" ^
//  -H "user-agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.141 Safari/537.36 Edg/87.0.664.75" ^
//  -H "accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*\/*;q=0.8,application/signed-exchange;v=b3;q=0.9" ^
//  -H "sec-fetch-site: none" ^
//  -H "sec-fetch-mode: navigate" ^
//  -H "sec-fetch-user: ?1" ^
//  -H "sec-fetch-dest: document" ^
//  -H "accept-language: en-US,en;q=0.9" ^
//  --compressed

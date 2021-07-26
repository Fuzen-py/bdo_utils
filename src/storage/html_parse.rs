// Parse the HTML from tooltips to get relevant info
// TODO: Validate Weight
// TODO: Validate Item Name
// TODO: Get Sell Price
// TODO: Get Buy Price

#[derive(Debug, Default)]
pub(crate) struct TempParsedTooltip {
    pub name: Option<String>,
    pub buy: Option<u128>,
    pub sell: Option<u128>,
    pub weight: Option<f32>,
}
pub(crate) struct ParsedToolTip {
    pub name: String,
    pub buy: Option<u128>,
    pub sell: Option<u128>,
    pub weight: f32,
}

pub(crate) fn parse_tooltip(html_body: String) -> Option<ParsedToolTip> {
    let mut res = TempParsedTooltip::default();

    for (ln, line) in html_body.replace("<br>", "\n").lines().enumerate() {
        if ln == 0 {
            continue;
        }
        //No KR name
        if line.contains("KR name:") {
            continue;
        }
        // Check if the line contains the item name
        // While not ideal, an id should only  be used once
        // If there is content after this is what we are looking for
        if let Some(name_section) = line.split("id=\"item_name\"").nth(1) {
            if let Some(n) = name_section.trim_start_matches("><b>").split('<').next() {
                res.name = Some(n.trim().to_owned());
            }
        }

        // Weight
        if let Some(pre_weight) = line.split("Weight: ").nth(1) {
            res.weight = pre_weight.replace("LT", "").trim().parse().ok();
        }

        // Buy Price
        if let Some(pre_buy) = line.split("Buy price: ").nth(1) {
            res.buy = pre_buy.replace(',', "").parse().ok();
        }

        // Sell Price
        if let Some(pre_sell) = line.split("Sell price: ").nth(1) {
            res.sell = pre_sell.replace(',', "").parse().ok();
        }
    }
    Some(ParsedToolTip {
        name: res.name?,
        buy: res.buy,
        sell: res.sell,
        weight: res.weight.unwrap_or_default(),
    })
}

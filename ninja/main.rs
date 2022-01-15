use http_req::request;
use std::env;

const API: &str = "https://poe.ninja/api/data/itemoverview";

fn main() {
    let league_name = arg(1);
    let item_type = arg(2);
    let price = arg(3).parse::<f32>().unwrap();

    let address = format!("{}?league={}&type={}", API, league_name, item_type);
    let mut writer = Vec::new();
    let _response = request::get(address, &mut writer).unwrap();
    let json = String::from_utf8(writer).unwrap();

    let mut cards_lt_price = Vec::new();
    let mut cards_ge_price = Vec::new();

    let mut card_name = Option::None;
    let mut card_value;

    for x in json.split(&[',', '{', '}'][..]) {
        if x.starts_with("\"name\"") {
            card_name = Some(x.split(':').nth(1).unwrap());
        } else if x.starts_with("\"chaosValue\"") {
            card_value = x.split(':').nth(1).unwrap().parse::<f32>().unwrap();
            if card_value < price {
                cards_lt_price.push(card_name.unwrap());
            } else {
                cards_ge_price.push(card_name.unwrap());
            }
        }
    }

    println!("Cards < {} chaos:", price);
    println!("{}", cards_lt_price.join(" "));
    println!();
    println!("Cards >= {} chaos:", price);
    println!("{}", cards_ge_price.join(" "));
}

fn arg(i: usize) -> String {
    env::args().nth(i).unwrap()
}

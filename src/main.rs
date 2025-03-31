use reqwest::blocking::get;
use serde_json;
use anyhow::Result;

fn get_post(post_id: usize) {
    let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json?", post_id);
    println!("{:?}", url);

}
fn main() -> Result<()> {

    // 1. Pull from hacker news api
    // - https://hacker-news.firebaseio.com/v0/topstories.json?print=pretty
    // - for each each story id, pull the data from the api
    // - https://hacker-news.firebaseio.com/v0/item/8863.json?
    // 2. Parase the data
    // 3. Sort alphabetically

    let url = "https://rubiehq.com/v0/topstories.json?print=pretty";

    let resp = get(url)?;

    let responseText = resp.text()?;


    let parsedPostIds = serde_json::from_str::<Vec<usize>>(&responseText)?;  
    println!("{:?}", parsedPostIds);
    get_post(parsedPostIds[0]);

    Ok(())
}

use reqwest::blocking::get;
use serde_json;
use anyhow::Result;
use news::get_posts;

fn get_post(post_id: usize) {
    let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json?", post_id);
    println!("{:?}", url);

}
fn main() -> Result<()> {

    // 1. Pull from hacker news api
    // - https://hacker-news.firebaseio.com/v0/topstories.json?print=pretty
    // 2. 


    // let url = "https://rubiehq.com/v0/topstories.json?print=pretty";

    // let resp = get(url)?;

    // let responseText = resp.text()?;


    // let parsedPostIds = serde_json::from_str::<Vec<usize>>(&responseText)?;  
    // println!("{:?}", parsedPostIds);
    // get_post(parsedPostIds[0]);


    // NEW VERSION:
    // 1. Pull from hacker news
    // 2. Parse the response JSON
    // 3. Get the title and URL for a random choosing of 10 stories

    // let title = "Test Title";
    // let URL = "http://hacker-news.com/test-title";

    // let random_posts = get_random_posts();

    // for post in random_posts {
    //     println!("{}, URL: {}", post.title, post.URL);
    // }

    for post in get_posts()? {
        println!("{}", post.title);
        let url = if let Some(post_url) = post.url {
            post.url
        } else {
            None
        }
        println!("{:?}", url);
    }

    Ok(())
}

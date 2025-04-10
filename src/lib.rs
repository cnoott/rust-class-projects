// functions
// - fetchPosts()
// - chooseRandomPosts()

use reqwest::blocking::get;
use anyhow::Result;
use serde_json::{self, json};
use serde_json::Value;
use anyhow::Context;

pub struct Post {
    pub title: String,
    pub url: Option<String>,
}

pub fn get_posts() -> Result<Vec<Post>> {
    let url = "https://hacker-news.firebaseio.com/v0/topstories.json?print=pretty";
    let resp_text = get(url)?.text()?;
    let parsed_post_ids = serde_json::from_str::<Vec<usize>>(&resp_text)?;

    let mut posts = Vec::new();

    for post_id in parsed_post_ids.iter().take(10) {
        let post_url = format!("https://hacker-news.firebaseio.com/v0/item/{post_id}.json?");
        let post = get(post_url)?.text()?;
        let json_post:Value = serde_json::from_str(&post)?;

        let post_url = if let Some(post_url_string) = json_post.pointer("/url") {
            Some(post_url_string.to_string())
        } else {
            None
        };

        let extracted_post = Post {
            title: json_post.pointer("/title").context("Post missing title!")?.to_string(),
            url: post_url,
        };

        posts.push(extracted_post);
    }

    Ok(posts)
}
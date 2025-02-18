// main.rs
use feed_rs::parser;
use reqwest;
use rss::{Channel, ChannelBuilder, Item, ItemBuilder};
use std::error::Error;
use std::fs;
use chrono::{DateTime, FixedOffset, Utc};
use tokio;
use serde::Deserialize;

// Config struct for deserializing config.toml
#[derive(Debug, Deserialize)]
struct Config {
    max_items: Option<usize>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Read configuration from config.toml
    let config: Config = fs::read_to_string("config.toml")
        .ok()
        .and_then(|contents| toml::from_str(&contents).ok())
        .unwrap_or(Config { max_items: None });
    let max_items = config.max_items.unwrap_or(300);
    println!("Using max_items = {}", max_items);

    // Read feed URLs from "feeds.txt" (one URL per line)
    let feeds_content = fs::read_to_string("feeds.txt")?;
    let feed_urls: Vec<String> = feeds_content
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|s| s.to_string())
        .collect();

    if feed_urls.is_empty() {
        eprintln!("No feed URLs found in feeds.txt");
        return Ok(());
    }

    // Concurrently fetch and parse feeds
    let mut all_items = Vec::new();
    let mut handles = Vec::new();
    for url in feed_urls {
        let url_owned = url.to_string();
        let handle = tokio::spawn(async move { fetch_feed_items(url_owned).await });
        handles.push(handle);
    }

    // Collect results from tasks
    for handle in handles {
        match handle.await? {
            Ok(mut feed_items) => all_items.append(&mut feed_items),
            Err(e) => eprintln!("Error fetching feed: {}", e),
        }
    }

    // Sort items by publication date (newest first)
    all_items.sort_by(|a, b| b.pub_date.cmp(&a.pub_date));

    // Limit the list to the maximum number of items specified
    if all_items.len() > max_items {
        all_items.truncate(max_items);
    }

    // Build a master RSS feed from the aggregated items
    let channel = build_master_feed(&all_items);

    // Write the generated RSS feed to a file
    fs::write("master_feed.xml", channel.to_string())?;
    println!("Master feed generated with {} items", all_items.len());

    Ok(())
}

// A simple struct to hold the feed item data
#[derive(Debug, Clone)]
struct FeedItem {
    title: String,
    link: String,
    description: Option<String>,
    pub_date: DateTime<FixedOffset>,
}

/// Fetches a feed from the given URL and parses its items.
async fn fetch_feed_items(url: String) -> Result<Vec<FeedItem>, Box<dyn Error + Send + Sync>> {
    let response = reqwest::get(&url).await?;
    let bytes = response.bytes().await?;
    let feed = parser::parse(bytes.as_ref())?;

    // Create a FixedOffset with zero offset.
    let offset = FixedOffset::east_opt(0).unwrap();

    let mut items = Vec::new();
    for entry in feed.entries {
        // Convert published/updated dates to DateTime<FixedOffset>
        let pub_date = entry
            .published
            .map(|d| d.with_timezone(&offset))
            .or(entry.updated.map(|d| d.with_timezone(&offset)))
            .unwrap_or_else(|| Utc::now().with_timezone::<FixedOffset>(&offset));

        // Use the first available link (if any)
        let link = if !entry.links.is_empty() {
            entry.links[0].href.clone()
        } else {
            String::new()
        };

        let title = entry
            .title
            .map(|t| t.content)
            .unwrap_or_else(|| String::from("No title"));

        let description = entry.summary.map(|s| s.content);

        items.push(FeedItem {
            title,
            link,
            description,
            pub_date,
        });
    }

    Ok(items)
}

/// Builds an RSS channel (master feed) from the provided items.
fn build_master_feed(items: &[FeedItem]) -> Channel {
    let rss_items: Vec<Item> = items
        .iter()
        .map(|fi| {
            let mut builder = ItemBuilder::default();
            builder.title(fi.title.clone());
            builder.link(fi.link.clone());
            if let Some(desc) = &fi.description {
                builder.description(desc.clone());
            }
            // Format the publication date as RFC 2822 for RSS
            builder.pub_date(fi.pub_date.to_rfc2822());
            builder.build()
        })
        .collect();

    ChannelBuilder::default()
        .title("Master RSS Feed")
        .link("https://yourrepo.github.io/master_feed.xml")
        .description("Aggregated RSS feed")
        .items(rss_items)
        .build()
}

# GitHub RSS Aggregator

Welcome to the **GitHub RSS Aggregator** – a simple, open-source, github-hosted RSS feed aggregator written in Rust. It fetches multiple RSS feeds concurrently, aggregates and sorts their items, and generates a single, master RSS feed file (`master_feed.xml`). It requires no backend or database—just pure goodness running completely in your Github repo!

## Features

- **Multi-Feed Support:** Reads feed URLs from a `feeds.txt` file (one URL per line).
- **Configurable:** Uses a `config.toml` file to set options (currently, the maximum number of items to sync).
- **Concurrent Fetching:** Uses asynchronous Rust with Tokio to fetch feeds in parallel.
- **Master Feed Creation:** Aggregates, deduplicates, and sorts feed items by publication date.
- **GitHub Actions Integration:**
   - **Build Release:** Automatically builds the release binary on pull requests.
   - **Update Master Feed:** Periodically runs the aggregator, updating `master_feed.xml` and pushing changes back to the repo.
   - **The GitHub link to the `master_feed.xml` file can be used in your favorite RSS reader directly**

## Usage

Just add urls (1 line per feed) to the `feeds.txt` file and commit the changes. That's it!
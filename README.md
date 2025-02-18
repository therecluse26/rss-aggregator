# GitHub RSS Aggregator

Welcome to the **GitHub RSS Aggregator** – a simple, open-source, GitHub-hosted RSS feed aggregator written in Rust. It fetches multiple RSS feeds concurrently, aggregates and sorts their items, and generates a single, master RSS feed file (`master_feed.xml`). It requires no backend or database—just pure goodness running completely in your GitHub repo!

## Features

- **Multi-Feed Support:** Reads feed URLs from a `feeds.txt` file (one URL per line).
- **Configurable:** Uses a `config.toml` file to set options (currently, the maximum number of items to sync).
- **Concurrent Fetching:** Uses asynchronous Rust with Tokio to fetch feeds in parallel.
- **Master Feed Creation:** Aggregates, deduplicates, and sorts feed items by publication date.
- **GitHub Actions Integration:**
  - **Build Release:** Automatically builds the release binary on pull requests.
  - **Update Master Feed:** Periodically runs the aggregator, updating `master_feed.xml` and pushing changes back to the repo.
  - **Direct RSS Link:** The GitHub link to the `master_feed.xml` file can be used in your favorite RSS reader directly.

## Usage

Just add URLs (one per line) to the `feeds.txt` file and commit the changes. That's it!

## Setting Up the Project with a PAT

For the GitHub Actions workflows to successfully create releases and push updates (such as updating `master_feed.xml`), you need to configure a Personal Access Token (PAT) and add it as a secret named `RELEASE_TOKEN` in your repository. This token is used by the workflows to authenticate operations that modify the repository.

### Required PAT Permissions

Your PAT must have the following scopes:
- **repo** — Grants full control of private repositories (and is sufficient for public repositories as well). This scope allows the workflows to:
  - Create and upload release assets.
  - Push changes (for example, updating `master_feed.xml`).

### Configuring the PAT

1. **Create a PAT:**
  - Go to your GitHub account's **Settings > Developer settings > Personal access tokens**.
  - Click on **Generate new token**.
  - Select the **repo** scope (and any additional scopes if needed) and generate the token.
  - **Copy the token** (you won’t be able to see it again).

2. **Add the PAT as a Repository Secret:**
  - In your repository, navigate to **Settings > Secrets and variables > Actions > New repository secret**.
  - Name the secret `RELEASE_TOKEN` and paste your PAT as its value.
  - Save the secret.

Once configured, the workflows will automatically use the `RELEASE_TOKEN` secret for release creation and pushing updates to the repository.

Enjoy aggregating your RSS feeds and happy coding! 🚀

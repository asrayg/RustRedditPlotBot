# RustRedditPlotBot

PlotBot is a sophisticated Reddit bot designed to create various types of data visualizations based on user input. It can fetch and visualize data provided inline or via a URL, and supports multiple plot types including line charts, bar charts, and scatter plots. PlotBot also includes advanced features like user configuration, caching, database integration, and enhanced error handling.

## Features

- **Multiple Plot Types**: Supports line charts, bar charts, and scatter plots.
- **Data Fetching**: Fetches data from inline comments or external URLs.
- **User Configuration**: Allows users to customize plot settings.
- **Database Integration**: Uses SQLite for storing processed comments and user preferences.
- **Caching**: Implements caching to avoid redundant data fetching.
- **Error Reporting**: Logs errors and can send error reports.
- **Metrics and Monitoring**: Collects metrics on bot performance and usage.

## Installation

1. **Clone the repository**:
   ```sh
   git clone https://github.com/asrayg/RustRedditPlotBot.git
   cd plotbot
   ```

2. **Set up the environment**:
   Create a `.env` file in the root directory and add your environment variables:
   ```env
   REDDIT_CLIENT_ID=your_reddit_client_id
   REDDIT_SECRET=your_reddit_secret
   REDDIT_USERNAME=your_reddit_username
   REDDIT_PASSWORD=your_reddit_password
   USER_AGENT=your_user_agent
   IMGUR_CLIENT_ID=your_imgur_client_id
   DATABASE_URL=sqlite://bot.db
   ```

3. **Install dependencies**:
   ```sh
   cargo build --release
   ```

4. **Initialize the database**:
   ```sh
   cargo run --release --bin plotbot -- init-db
   ```

## Usage

Run the bot with the following command:
```sh
cargo run --release
```

Alternatively, you can use `systemd` to run the bot continuously on a server. Create a `plotbot.service` file with the following content:

```ini
[Unit]
Description=PlotBot - Reddit Data Visualization Bot
After=network.target

[Service]
User=your_username
WorkingDirectory=/path/to/your/plotbot
ExecStart=/path/to/your/plotbot/target/release/plotbot
Restart=always

[Install]
WantedBy=multi-user.target
```

Enable and start the service:

```sh
sudo systemctl enable plotbot
sudo systemctl start plotbot
```

## Commands

- **!visualize line <data>**: Create a line chart from the provided data.
- **!visualize bar <data>**: Create a bar chart from the provided data.
- **!visualize scatter <data>**: Create a scatter plot from the provided data.
- **!visualize line <url>**: Create a line chart from data fetched from the specified URL.
- **!visualize bar <url>**: Create a bar chart from data fetched from the specified URL.
- **!visualize scatter <url>**: Create a scatter plot from data fetched from the specified URL.

## Configuration

You can customize the bot by editing the `.env` file to set your Reddit and Imgur API credentials, user agent, and database URL.

## Contributing

We welcome contributions! Please open an issue or submit a pull request on GitHub.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [Rust](https://www.rust-lang.org/)
- [Plotters](https://crates.io/crates/plotters)
- [Reqwest](https://crates.io/crates/reqwest)
- [SQLx](https://crates.io/crates/sqlx)
- [dotenv](https://crates.io/crates/dotenv)
- [log](https://crates.io/crates/log)
- [env_logger](https://crates.io/crates/env_logger)

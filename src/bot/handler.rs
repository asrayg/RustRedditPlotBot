use crate::imgur::upload_image_to_imgur;
use crate::plots::{generate_bar_plot, generate_line_plot, generate_scatter_plot};
use crate::reddit::reply_to_comment;
use crate::bot::command::{Command, PlotType, Data};
use log::info;
use reqwest::Client;
use std::error::Error;

pub struct Handler {
    imgur_client: Client,
    imgur_client_id: String,
}

impl Handler {
    pub async fn new(imgur_client_id: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            imgur_client: Client::new(),
            imgur_client_id: imgur_client_id.to_string(),
        })
    }

    pub async fn handle_command(&self, reddit_client: &Client, access_token: &str, user_agent: &str, comment: &crate::reddit::Comment, command: &Command) -> Result<(), Box<dyn Error>> {
        let data = match &command.data {
            Data::Inline(data) => data.clone(),
            Data::Url(url) => self.fetch_data_from_url(url).await?,
        };

        let image_path = match command.plot_type {
            PlotType::Line => generate_line_plot(&data),
            PlotType::Bar => generate_bar_plot(&data),
            PlotType::Scatter => generate_scatter_plot(&data),
        }?;

        let image_url = upload_image_to_imgur(&image_path, &self.imgur_client, &self.imgur_client_id).await?;
        reply_to_comment(reddit_client, access_token, user_agent, &comment.id, &image_url).await?;

        info!("Generated plot for comment ID: {}", comment.id);

        Ok(())
    }

    async fn fetch_data_from_url(&self, url: &str) -> Result<Vec<f64>, Box<dyn Error>> {
        let res = self.imgur_client.get(url).send().await?;
        let text = res.text().await?;
        let data: Vec<f64> = text.split_whitespace().filter_map(|s| s.parse().ok()).collect();

        Ok(data)
    }
}

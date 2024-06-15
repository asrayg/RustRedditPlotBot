use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize)]
pub struct Comment {
    pub body: String,
    pub id: String,
    pub link_id: String,
}

#[derive(Serialize, Deserialize)]
struct ApiResponse {
    data: CommentData,
}

#[derive(Serialize, Deserialize)]
struct CommentData {
    children: Vec<Child>,
}

#[derive(Serialize, Deserialize)]
struct Child {
    data: Comment,
}

pub async fn fetch_comments(client: &Client, access_token: &str, user_agent: &str) -> Result<Vec<Comment>, Box<dyn Error>> {
    let comments = client
        .get("https://oauth.reddit.com/r/your_subreddit/comments")
        .bearer_auth(access_token)
        .header("User-Agent", user_agent)
        .send()
        .await?;

    let comments_text = comments.text().await?;
    let api_response: ApiResponse = serde_json::from_str(&comments_text)?;

    Ok(api_response.data.children.into_iter().map(|child| child.data).collect())
}

pub async fn reply_to_comment(client: &Client, access_token: &str, user_agent: &str, comment_id: &str, message: &str) -> Result<(), Box<dyn Error>> {
    let params = [("thing_id", comment_id), ("text", message)];
    
    client
        .post("https://oauth.reddit.com/api/comment")
        .bearer_auth(access_token)
        .header("User-Agent", user_agent)
        .form(&params)
        .send()
        .await?;

    Ok(())
}
use reqwest::{Client, Error};

use crate::models::{Repository, User};

pub struct GithubClient {
    client: Client,
    base_url: String,
}

impl GithubClient {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            base_url: String::from("https://api.github.com"),
        }
    }

    pub async fn get_user(&self, username: &str) -> Result<User, Error> {
        let response = self
            .client
            .get(format!("{}/users/{}", self.base_url, username))
            .header("User-Agent", "github-cli")
            .send()
            .await?;

        response.error_for_status()?.json::<User>().await
    }

    pub async fn get_repositories(&self, username: &str) -> Result<Vec<Repository>, Error> {
        let response = self
            .client
            .get(format!("{}/users/{}/repos", self.base_url, username))
            .header("User-Agent", "github-cli")
            .send()
            .await?;
        
        response.error_for_status()?.json::<Vec<Repository>>().await
    }
}
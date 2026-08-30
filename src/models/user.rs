use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct User {
    pub name: Option<String>,
    pub login: String,
    pub bio: Option<String>,
    pub public_repos: u32,
    pub followers: u32,
    pub following: u32,
}
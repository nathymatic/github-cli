use jstdlib::{console::Console, write_line};
use reqwest::Client;

use crate::api::GithubClient;

pub async fn run() {
    let client = GithubClient::new(Client::new());

    loop {
        if !main_loop(&client).await {
            break;
        }


    }
}

pub async fn main_loop(client: &GithubClient) -> bool {
    let username: String = Console::input("GitHub username: ");

    match client.get_user(&username).await {
        Ok(user) => {
            write_line!("Name: {}", user.name.as_deref().unwrap_or("N/A"));
            write_line!("Username: {}", user.login);
            write_line!("Bio: {}", user.bio.as_deref().unwrap_or("N/A"));
            write_line!("Public repos count: {}", user.public_repos);
            write_line!("Followers count: {}", user.followers);
            write_line!("Following count: {}", user.following);

            match client.get_repositories(&username).await {
                Ok(repositories) => {
                    write_line!("\nRepositories:");

                    for repository in repositories {
                        write_line!("\n- {}", repository.name);
                        write_line!(
                            "  Description: {}",
                            repository.desc.as_deref().unwrap_or("N/A")
                        );
                        write_line!("  URL: {}", repository.html_url);
                        write_line!("  Stars: {}", repository.stargazers_count);
                        write_line!("  Forks: {}", repository.forks_count);
                    }
                }
                Err(err) => {
                    write_line!("[Error] Failed to fetch repositories: {}", err);
                }
            }
        }
        Err(err) => {
            write_line!("[Error] {}", err);
        }
    }

    Console::confirm("Search another user? (y/n): ")
}
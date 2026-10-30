use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Repository {
    pub name: String,
    pub desc: Option<String>,
    pub html_url: String,
    pub stargazers_count: u32,
    pub forks_count: u32,
}

/* ?
<style>
    body {
        background-image: url("StactyReeseCMendoza.9F.MjrPTinComp&AP\1.png");
        background-size: cover;
        background-attachment: fixed;
        background-color: #343434;

        margin: 0;
        height: 100vh;
        color: white;
        font-family: sans-serif;
    }
</style>
*/
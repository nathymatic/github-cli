# github-cli

A small Rust CLI for fetching GitHub user profiles and public repositories.

Built as a Rust learning project focused on REST APIs, asynchronous programming, JSON deserialization, and CLI structure.

## Features

* Fetch GitHub user profiles
* List public repositories
* Display repository descriptions, URLs, stars, and forks
* Handles invalid usernames
* Async HTTP requests with `reqwest` and `tokio`
* JSON deserialization with `serde`
* Console I/O with `jstdlib`

## Stack Used

* Rust
* Reqwest
* Serde
* Tokio
* jstdlib

## Structure

```text
src/
├── main.rs
├── app.rs
├── api/
│   ├── mod.rs
│   └── github_client.rs
└── models/
    ├── mod.rs
    ├── user.rs
    └── repository.rs
```

## Running

```bash
cargo run
```

For a release build:

```bash
cargo build --release
```

## Example

```text
GitHub username: jayywashere

Name: jayyzor
Username: jayywashere
Bio: ...
Public repos count: 11
Followers count: 0
Following count: 0

Repositories:

- jstd
  Description: N/A
  URL: https://github.com/jayywashere/jstd
  Stars: 0
  Forks: 0
```

## What I Learned (And Will Forget Tomorrow)

* Rust modules and structs
* `Option<T>` and `Result<T, E>`
* Error handling
* `async` / `await`
* HTTP requests and REST APIs
* JSON deserialization with Serde
* Working with `Vec<T>`
* Using a personal Rust utility crate
* Separating application, API, and model logic
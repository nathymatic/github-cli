# github-cli

> a small Rust CLI for fetching GitHub profiles and public repositories because apparently i wanted to make my own tiny GitHub API thing thingy thing

## About

so uhm, basically, this is a little Rust project i made to mess around with REST APIs, async stuff, JSON deserialization, and making an actually structured CLI

it uses GitHub's public API to fetch a user's profile and repositories...

## Features

* fetch GitHub user profiles
* list public repositories
* show repository descriptions, URLs, stars, and forks
* handles invalid usernames
* async HTTP requests with `reqwest` and `tokio`
* JSON deserialization with `serde`
* console I/O with `jstdlib`

## Requirements

* Rust
* Cargo

## Installation

clone the repository:

```bash
git clone <insert-repository-url-here>
cd github-cli
```

then run it with Cargo:

```bash
cargo run
```

or build a release version:

```bash
cargo build --release
```

the compiled executable will be in `target/release/`.

## Example

```text
GitHub username: nathymatic

Name: jayyzor
Username: nathymatic
Bio: ...

Public repos count: 11
Followers count: 0
Following count: 0

Repositories:

- jstd
  Description: N/A
  URL: https://github.com/nathymatic/jstd
  Stars: 0
  Forks: 0
```

## what i learned (and will probably forget tmrw)

* Rust modules and structs, ofc
* `Option<T>` and `Result<T, E>`. WOOHOOOOOOOOOOOOOOOO
* error handling
* `async` / `await`
* HTTP requests and REST APIs
* JSON deserialization with Serde
* working with `Vec<T>`
* using a personal Rust utility crate
* separating application, API, and model logic

## License

See [LICENSE](LICENSE).
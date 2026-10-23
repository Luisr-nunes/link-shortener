# Link Shortener

<!-- badges -->
![Rust](https://img.shields.io/badge/Rust-000000?style=flat-square&logo=rust&logoColor=white) ![Axum](https://img.shields.io/badge/Axum-4A4A4A?style=flat-square) ![Tokio](https://img.shields.io/badge/Tokio-4A4A4A?style=flat-square) ![PostgreSQL](https://img.shields.io/badge/PostgreSQL-4169E1?style=flat-square&logo=postgresql&logoColor=white) ![HTML5](https://img.shields.io/badge/HTML5-E34F26?style=flat-square&logo=html5&logoColor=white) ![CSS3](https://img.shields.io/badge/CSS3-663399?style=flat-square&logo=css&logoColor=white) ![JavaScript](https://img.shields.io/badge/JavaScript-F7DF1E?style=flat-square&logo=javascript&logoColor=black)

[Ler em Português](README.md)

URL shortener in Rust with Axum and PostgreSQL: a REST API to create short links, redirects with a click counter and a simple web UI served by the binary itself.

## Features

- Generates a random 6-character alphanumeric code for each URL.
- Redirects `/r/<code>` to the original URL and increments the click counter.
- Applies database migrations automatically on startup.
- Web UI in `static/index.html`, served at the root.

## Tech stack

| Layer | Technology |
| --- | --- |
| Language | Rust (2021 edition) |
| Web | Axum 0.7, Tokio, `tower-http` (static files) |
| Database | PostgreSQL with SQLx 0.7 (async queries and migrations) |
| Configuration | `dotenvy` |
| UI | HTML, CSS and JavaScript |

## Structure

```
link-shortener/
├── src/
│   ├── main.rs       # connection, migrations, routes and server
│   ├── handlers.rs   # shorten and redirect
│   ├── models.rs     # request and response structs
│   └── db.rs
├── migrations/       # SQLx migration for the links table
├── schema.sql        # same schema, for manual setup
├── static/index.html
└── Cargo.toml
```

## Running

Requirements: Rust (rustup) and PostgreSQL.

```bash
git clone https://github.com/Luisr-nunes/link-shortener.git
cd link-shortener

createdb link_shortener
echo "DATABASE_URL=postgres://user:password@localhost:5432/link_shortener" > .env

cargo run
```

The server starts on `http://127.0.0.1:3000` and creates the `links` table on first run.

## API

### Shorten a URL

`POST /api/shorten`

```json
{ "long_url": "https://www.rust-lang.org" }
```

Response `201 Created`:

```json
{ "id": 1, "short_code": "aB3xY9", "long_url": "https://www.rust-lang.org", "clicks": 0 }
```

### Redirect

`GET /r/:short_code` redirects to the original URL and adds one click.

## Author

Luís Nunes — [GitHub](https://github.com/Luisr-nunes) · [LinkedIn](https://www.linkedin.com/in/luis-nunes-dev/)

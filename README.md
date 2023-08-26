# Scaffold Rocket REST API with Diesel - MySQL


## Requirements
- [Rust]((https://rustup.rs/))
- [MySQL](https://www.mysql.com/)
- [Diesel](https://diesel.rs/guides/getting-started)

## How to run

### Manual
- Install Rust nightly (Thanks for Rustup 1.20!): `rustup install nightly`
- Set Rust Nightly to project: Go to the root of the project, open cmd/terminal and run `rustup override set nightly`
- Rename `secret.key.sample` to `secret.key` or create your own key by running `head -c16 /dev/urandom > secret.key` in command line (Linux/UNIX only) and copy to `/src` folder
- Create a database in mysql workbench/cli
- Rename `env.example` to `.env` and update the database connection string in `DATABASE_URL` key.
- Build with debug profile: `cargo watch -q -c -w src/ -x run`
- Or Build with release profile: `cargo build --release`
- Enjoy! 😄
### Docker
- Enter into project directory and run `docker-compose up -d`
- Enjoy! 😄
### Or with Makefile if you are in Linux
- Developing .. 😄

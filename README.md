# Noice
An eve online accounting tool
## Installation
- `git clone` this project
- copy `.env.example` to `.env`
- modify `.env`
- install diesel-cli `cargo install diesel-cli`
- install cargo-web `cargo install cargo-web`
- create database and run migrations `diesel setup`

## Running
### Server side
- copy `Rocket.toml.example` to `Rocket.toml`
- modify `Rocket.toml`
- in server folder `cargo run`

### Things to be done
- buy orders have their broker fee too!
- sell orders have taxes
- sell orders don't have broker fees right? 
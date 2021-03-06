#[macro_use]
extern crate derive_builder;

mod cli;
mod context;
mod discord;
mod env;
mod openai;
mod stdio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  cli::match_cmd(cli::app().get_matches()).await;
  Ok(())
}

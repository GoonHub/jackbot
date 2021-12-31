#[macro_use]
extern crate derive_builder;

mod context;

use context::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut context = build_context();

  context.add_message("insprac".into(), "Hello, what should I do today?".into());

  println!("{}", context.completion().await?);

  println!("{}", context.prompt());
  println!("{}", context);

  Ok(())
}

fn build_context() -> Context {
  let api_token = std::env::var("JACK_OPENAI_KEY").unwrap();
  let base_context = std::env::var("JACK_BASE_CONTEXT").unwrap();
  let bot_name = std::env::var("JACK_BOT_NAME").unwrap();
  let client = openai_api::Client::new(&api_token);

  let mut context = Context::builder()
    .client(client)
    .engine(openai_api::api::Engine::Ada)
    .temperature(0.7)
    .build()
    .unwrap();

  if !base_context.is_empty() {
    context.base = base_context;
  }

  if !bot_name.is_empty() {
    context.bot_name = bot_name;
  }

  context
}

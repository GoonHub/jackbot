use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
  macros::{command, group},
  CommandResult, StandardFramework,
};
use serenity::model::channel::Message;

use std::env;

#[group]
#[commands(jack)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

struct JackContext;

impl serenity::prelude::TypeMapKey for JackContext {
  type Value = crate::context::Context;
}

pub async fn bot() {
  let framework = StandardFramework::new()
    .configure(|c| c.prefix("!"))
    .group(&GENERAL_GROUP);

  let token = env::var("JACK_DISCORD_TOKEN").expect("token");
  let mut client = Client::builder(token)
    .event_handler(Handler)
    .framework(framework)
    .await
    .expect("Error creating client");

  if let Err(why) = client.start().await {
    println!("An error occurred while running the client: {:?}", why);
  }
}

#[command]
async fn jack(ctx: &Context, msg: &Message) -> CommandResult {
  let mut context = crate::context::from_env(msg.channel_id.to_string());

  for mut message in msg.content.split("\n") {
    message = message.trim_start_matches("!jack ");
    context.add_message(msg.author.name.clone(), message.into());
  }

  msg.reply(ctx, context.completion().await?).await?;

  context.write_messages();

  Ok(())
}

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
  macros::{command, group},
  CommandResult, StandardFramework,
};
use serenity::model::channel::Message;

#[group]
#[commands(jack, more, reset, context, set_context, reset_context)]
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

  let token = crate::env::discord_token();
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

  for mut message in msg.content.trim().split("\n") {
    message = message.trim_start_matches("!jack ");
    context.add_message(msg.author.name.clone(), message.into());
  }

  let completion = context.completion().await.unwrap();
  msg.reply(ctx, completion).await.unwrap();
  context.write_messages();

  Ok(())
}

#[command]
async fn more(ctx: &Context, msg: &Message) -> CommandResult {
  let mut context = crate::context::from_env(msg.channel_id.to_string());
  let completion = context.completion().await.unwrap();
  msg.reply(ctx, completion).await.unwrap();
  context.write_messages();

  Ok(())
}

#[command]
async fn reset(ctx: &Context, msg: &Message) -> CommandResult {
  let context = crate::context::from_env(msg.channel_id.to_string());
  context.reset_messages();

  msg.reply(ctx, "Good as new :slight_smile:").await.unwrap();

  Ok(())
}

#[command]
async fn context(ctx: &Context, msg: &Message) -> CommandResult {
  let context = crate::context::from_env(msg.channel_id.to_string());
  msg.reply(ctx, context.read_base_context()).await.unwrap();
  Ok(())
}

#[command]
async fn set_context(ctx: &Context, msg: &Message) -> CommandResult {
  let context = crate::context::from_env(msg.channel_id.to_string());
  context.write_base_context(&msg.content.replace("!set_context ", ""));

  msg
    .reply(ctx, "Context updated :writing_hand:")
    .await
    .unwrap();

  Ok(())
}

#[command]
async fn reset_context(ctx: &Context, msg: &Message) -> CommandResult {
  let context = crate::context::from_env(msg.channel_id.to_string());
  context.reset_context();
  msg
    .reply(ctx, "Context reset :slight_smile:")
    .await
    .unwrap();
  Ok(())
}

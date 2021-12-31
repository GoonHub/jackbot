use std::fmt::{self, Display, Formatter};

#[derive(Builder, Clone)]
pub struct Context {
  #[builder(
    setter(into),
    default = "\"The following is a conversation with Jack who is friendly, clever and loves to give advice.\".into()"
  )]
  pub base: String,
  #[builder(default = "20")]
  pub max_replies: u16,
  #[builder(default = "50")]
  pub max_tokens: u64,
  #[builder(default = "1.0")]
  pub temperature: f64,
  #[builder(setter(into), default = "\"Jack\".into()")]
  pub bot_name: String,
  #[builder(default = "openai_api::Client::new(\"\")")]
  client: openai_api::Client,
  #[builder(default = "openai_api::api::Engine::Davinci")]
  engine: openai_api::api::Engine,
  #[builder(default = "vec![]")]
  messages: Vec<Message>,
}

impl Context {
  #[must_use]
  pub fn builder() -> ContextBuilder {
    ContextBuilder::default()
  }

  pub async fn completion(&mut self) -> Result<String, openai_api::Error> {
    let args = openai_api::api::CompletionArgs::builder()
      .prompt(self.clone().prompt())
      .engine(self.engine)
      .max_tokens(self.max_tokens)
      .temperature(self.temperature)
      .stop(vec!["\n".into(), ".".into()])
      .build()?;

    let completion = self.client.complete_prompt(args).await?;
    let text: String = format!("{}", completion.choices[0].text).trim().into();

    if !text.is_empty() && !text.eq("?") {
      self.add_message(self.bot_name.clone(), text.clone());
    }

    if text.is_empty() {
      Ok("?".into())
    } else {
      Ok(text)
    }
  }

  pub fn add_message(&mut self, sender: String, content: String) {
    self.messages.push(
      Message::builder()
        .sender(sender)
        .content(content)
        .build()
        .unwrap(),
    );
  }

  pub fn prompt(&self) -> String {
    let mut prompt = format!("{}\n\n", self.base);

    for message in &self.messages {
      prompt.push_str(format!("{}: {}\n", message.sender, message.content).as_ref());
    }

    format!("{}{}:", self, self.bot_name)
  }
}

impl Display for Context {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}\n\n", self.base)?;

    for message in &self.messages {
      write!(f, "{}: {}\n", message.sender, message.content)?;
    }

    Ok(())
  }
}

#[derive(Builder, Clone)]
pub struct Message {
  #[builder(setter(into), default = "\"\".into()")]
  sender: String,
  #[builder(setter(into), default = "\"\".into()")]
  content: String,
}

impl Message {
  #[must_use]
  pub fn builder() -> MessageBuilder {
    MessageBuilder::default()
  }
}

use std::fmt::{self, Display, Formatter};

#[derive(Builder, Clone)]
pub struct Context {
  #[builder(setter(into), default = "\"default\".into()")]
  pub id: String,
  #[builder(
    setter(into),
    default = "\"The following is a conversation with Jack who is friendly, clever and loves to give advice.\".into()"
  )]
  pub base: String,
  #[builder(default = "20")]
  pub max_replies: u16,
  #[builder(default = "50")]
  pub max_tokens: u16,
  #[builder(default = "1.0")]
  pub temperature: f32,
  #[builder(setter(into), default = "\"Jack\".into()")]
  pub bot_name: String,
  #[builder(default = "\"ada\".into()")]
  engine: String,
  #[builder(default = "vec![]")]
  messages: Vec<Message>,
}

impl Context {
  #[must_use]
  pub fn builder() -> ContextBuilder {
    ContextBuilder::default()
  }

  pub async fn completion(&mut self) -> Result<String, Box<dyn std::error::Error>> {
    let args = crate::openai::CompletionArgs::builder()
      .prompt(self.clone().prompt())
      .engine(self.engine.clone())
      .max_tokens(self.max_tokens)
      .temperature(self.temperature)
      .stop(vec!["\n".into(), ".".into()])
      .build()?;

    let completion = crate::openai::completion(&args).await.unwrap();
    let text: String = format!("{}", completion.choices[0].text).trim().into();

    if !text.is_empty() && !text.eq("?") {
      self.add_message(self.bot_name.clone(), text.clone());
    }

    println!("{}", self);

    if text.is_empty() {
      Ok("?".into())
    } else {
      Ok(text)
    }
  }

  pub fn add_message(&mut self, sender: String, content: String) {
    self.messages.push(
      Message::builder()
        .sender(sender.trim())
        .content(content.trim())
        .build()
        .unwrap(),
    );
  }

  pub fn prompt(&self) -> String {
    let mut prompt = format!("{}\n\n", self.base);
    let trimmed_messages = self
      .messages
      .iter()
      .rev()
      .take(self.max_replies.into())
      .rev();

    for message in trimmed_messages {
      prompt.push_str(format!("{}: {}\n", message.sender, message.content).as_ref());
    }

    format!("{}{}:", self, self.bot_name)
  }

  pub fn read_messages(&mut self) {
    match std::fs::read_to_string(self.file_path()) {
      Err(_) => (),
      Ok(contents) => {
        for line in contents.trim().split("\n") {
          let mut line_split: Vec<&str> = line.split(": ").collect();

          if line_split.len() == 1 {
            self.add_message("".into(), line_split[0].into());
          } else if line_split.len() > 1 {
            let name = line_split[0];
            line_split.remove(0);
            self.add_message(name.into(), line_split.join(": "));
          }
        }
      }
    }
  }

  pub fn write_messages(&self) {
    let mut contents: String = "".into();
    for message in &self.messages {
      contents.push_str(&format!("{}: {}\n", message.sender, message.content))
    }

    std::fs::write(self.file_path(), contents).unwrap();
  }

  fn file_path(&self) -> String {
    let path = std::env::var("JACK_PATH").unwrap();
    if !std::path::Path::new(&path).exists() {
      panic!("JACK_PATH must be an existing folder")
    }

    std::path::Path::new(&path)
      .join(self.id.clone())
      .to_str()
      .unwrap()
      .into()
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

pub fn from_env(id: String) -> Context {
  let base_context = std::env::var("JACK_BASE_CONTEXT").unwrap();
  let bot_name = std::env::var("JACK_BOT_NAME").unwrap();

  let mut context = Context::builder()
    .id(id)
    .engine("babbage".into())
    .temperature(0.7)
    .build()
    .unwrap();

  if !base_context.is_empty() {
    context.base = base_context;
  }

  if !bot_name.is_empty() {
    context.bot_name = bot_name;
  }

  context.read_messages();

  context
}

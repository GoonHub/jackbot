use std::fmt::{self, Display, Formatter};
use std::time::SystemTime;

#[derive(Builder, Clone)]
pub struct Context {
  #[builder(setter(into), default = "\"default\".into()")]
  pub id: String,
  #[builder(default = "50")]
  pub max_replies: u16,
  #[builder(default = "50")]
  pub max_tokens: u16,
  #[builder(default = "0.7")]
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
      .prompt(self.prompt())
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

    self.write_messages();
  }

  pub fn prompt(&self) -> String {
    format!("{}{}:", self, self.bot_name)
  }

  pub fn read_messages(&mut self) {
    match std::fs::read_to_string(self.message_file_path()) {
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

    std::fs::write(self.message_file_path(), contents).unwrap();
  }

  pub fn write_base_context(&self, base_context: &str) {
    self.reset_context();
    std::fs::write(self.context_file_path(), base_context).unwrap();
  }

  pub fn read_base_context(&self) -> String {
    match std::fs::read_to_string(self.context_file_path()) {
      Err(_) => crate::env::base_context(),
      Ok(contents) => contents,
    }
  }

  pub fn reset_messages(&self) {
    let path = self.message_file_path();
    let time = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)
      .unwrap()
      .as_secs();
    let target = format!("{}.{}", path, time);
    std::fs::rename(path, target).unwrap();
  }

  pub fn reset_context(&self) {
    let path = self.context_file_path();
    let time = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)
      .unwrap()
      .as_secs();
    let target = format!("{}.{}", path, time);
    std::fs::rename(path, target).unwrap();
  }

  fn message_file_path(&self) -> String {
    return self.file_path("messages".into());
  }

  fn context_file_path(&self) -> String {
    return self.file_path("contexts".into());
  }

  fn file_path(&self, prefix: String) -> String {
    let path = crate::env::path();
    if !std::path::Path::new(&path).exists() {
      panic!("JACK_PATH must be an existing folder")
    }

    if !std::path::Path::new(&path).join(&prefix).exists() {
      panic!("{}/{} must be an existing folder", &path, &prefix);
    }

    std::path::Path::new(&path)
      .join(&prefix)
      .join(self.id.clone())
      .to_str()
      .unwrap()
      .into()
  }
}

impl Display for Context {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}\n\n", self.read_base_context())?;
    let trimmed_messages = self
      .messages
      .iter()
      .rev()
      .take(self.max_replies.into())
      .rev();

    for message in trimmed_messages {
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
  let bot_name = crate::env::bot_name();
  let engine = crate::env::engine();

  let mut context = Context::builder().id(id).engine(engine).build().unwrap();

  if !bot_name.is_empty() {
    context.bot_name = bot_name;
  }

  context.read_messages();

  context
}

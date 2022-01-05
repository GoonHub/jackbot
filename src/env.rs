pub fn discord_token() -> String {
  var("DISCORD_TOKEN", "".into())
}

pub fn bot_name() -> String {
  var("BOT_NAME", "Jack".into())
}

pub fn base_context() -> String {
  var(
    "BASE_CONTEXT",
    "The following is a conversation with Jack who is friendly, clever and loves to give advice."
      .into(),
  )
}

pub fn engine() -> String {
  var("ENGINE", "curie".into())
}

pub fn path() -> String {
  var("PATH", "".into())
}

pub fn openai_key() -> String {
  var("OPENAI_KEY", "".into())
}

fn var(name: &str, default: String) -> String {
  let mut var_name: String = "JACK_".into();
  var_name.push_str(name);
  match std::env::var(&var_name) {
    Err(_) => {
      if default == "" {
        panic!("Environment variable not set: {}", &var_name)
      } else {
        default
      }
    }
    Ok(value) => {
      if value.is_empty() && default == "" {
        panic!("Environment variable not set: {}", &var_name)
      } else if value.is_empty() {
        default
      } else {
        value
      }
    }
  }
}

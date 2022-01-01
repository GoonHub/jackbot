use reqwest::Client;

pub struct OpenAIClient {
  http_client: Client,
  api_token: String,
}

#[derive(Builder)]
pub struct CompletionArgs {
  #[builder(default = "1.0")]
  temperature: f32,
  #[builder(default = "50")]
  max_tokens: u16,
  #[builder(default = "vec![\"\n\".into()]")]
  stop: Vec<String>,
  #[builder(default = "\"ada\".into()")]
  engine: String,
}

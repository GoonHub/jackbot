use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

const BASE_URL: &str = "https://api.openai.com/v1";

#[derive(Builder, Serialize)]
pub struct CompletionArgs {
  pub prompt: String,
  #[builder(default = "1.0")]
  pub temperature: f32,
  #[builder(default = "50")]
  pub max_tokens: u16,
  #[builder(default = "vec![\"\n\".into()]")]
  pub stop: Vec<String>,
  #[serde(skip_serializing)]
  #[builder(default = "\"ada\".into()")]
  pub engine: String,
}

impl CompletionArgs {
  #[must_use]
  pub fn builder() -> CompletionArgsBuilder {
    CompletionArgsBuilder::default()
  }
}

#[derive(Deserialize)]
pub struct Completion {
  pub id: String,
  pub created: u64,
  pub model: String,
  pub choices: Vec<CompletionChoice>,
}

#[derive(Deserialize)]
pub struct CompletionChoice {
  pub text: String,
  pub index: u64,
  pub finish_reason: String,
}

pub async fn completion(
  args: &CompletionArgs,
) -> std::result::Result<Completion, Box<dyn std::error::Error>> {
  let url = format!("{}/engines/{}/completions", BASE_URL, args.engine);
  let client = reqwest::Client::new();
  let body = serde_json::to_string(args).unwrap();

  let api_token = crate::env::openai_key();
  let response = client
    .post(url)
    .body(body.clone())
    .header(AUTHORIZATION, format!("Bearer {}", api_token))
    .header(CONTENT_TYPE, "application/json")
    .send()
    .await
    .unwrap();

  let result: Completion = response.json().await.unwrap();
  Ok(result)
}

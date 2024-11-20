use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::{json, Value};
use std::error::Error;

pub async fn chat(
    api_key: String,
    prompt: &str,
    history: Vec<Value>,
) -> Result<String, Box<dyn Error>> {
    let url = "https://api.openai.com/v1/chat/completions";

    let mut chat_history = history;

    chat_history.push(json!({
        "role": "user",
        "content": prompt
    }));

    let body = serde_json::json!({
        "model": "gpt-4o-mini",
        "messages": chat_history,
        "temperature": 0.7
    });

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", api_key.as_str()))?,
    );

    let client = reqwest::Client::new();
    let response = client.post(url).headers(headers).json(&body).send().await?;

    if response.status().is_success() {
        let json_response: Value = response.json().await?;
        if let Some(content) = json_response["choices"]
            .get(0)
            .and_then(|choice| choice["message"]["content"].as_str())
        {
            Ok(content.to_string())
        } else {
            Err("Não foi possível encontrar o conteúdo da resposta.".into())
        }
    } else {
        Err(format!("Erro: {:?}", response.text().await?).into())
    }
}

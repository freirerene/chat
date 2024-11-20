use crate::database::database::{history, register_prompt};
use crate::openai::api::chat;
use anyhow::Result;
use tui_textarea::TextArea;

pub fn find_query(val: Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    if let Some((index, _)) = val.iter().enumerate().rev().find(|&(_, line)| {
        line == "==============================================================================="
    }) {
        result = val.into_iter().skip(index + 1).collect();
    }
    result
}

pub async fn query_response(textarea: &mut TextArea<'_>, api_key: String) -> Result<()> {
    textarea.insert_newline();
    textarea.insert_str(
        "===============================================================================",
    );
    let text_vec = textarea.lines();
    let query_text = find_query(text_vec.to_vec());
    let text: String;
    if query_text.len() > 0 {
        text = find_query(text_vec.to_vec()).join("\n");
    } else {
        text = text_vec.join("\n");
    }
    let chat_history = history().await?;
    match chat(api_key, &text, chat_history).await {
        Ok(response) => {
            let _ = register_prompt(&text, &response).await;
            textarea.insert_newline();
            textarea.insert_str(&response);
            textarea.insert_newline();
            textarea.insert_str(
                "===============================================================================",
            );
            textarea.insert_newline();
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            return Err(anyhow::Error::msg(e.to_string()));
        }
    }
    Ok(())
}

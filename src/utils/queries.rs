use crate::backend::database::{history, register_prompt};
use crate::llms::api::chat;
use anyhow::Result;
use tui_textarea::TextArea;

const SEPARATOR: &str =
    "===============================================================================";

pub async fn query_response(textarea: &mut TextArea<'_>, api_key: String) -> Result<()> {
    textarea.insert_newline();
    textarea.insert_str(SEPARATOR);
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
            textarea.insert_str(format_text(&response));
            textarea.insert_newline();
            textarea.insert_str(SEPARATOR);
            textarea.insert_newline();
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            return Err(anyhow::Error::msg(e.to_string()));
        }
    }
    Ok(())
}

fn find_query(val: Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    if let Some((index, _)) = val
        .iter()
        .enumerate()
        .rev()
        .find(|&(_, line)| line == SEPARATOR)
    {
        result = val.into_iter().skip(index + 1).collect();
    }
    result
}

fn format_text(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            if line.chars().count() > 100 {
                let mut formatted_line = String::new();
                let mut current_line_length = 0;

                for word in line.split_whitespace() {
                    let word_length = word.chars().count();

                    if current_line_length + word_length + 1 > 100 {
                        formatted_line.push('\n');
                        current_line_length = 0;
                    } else if current_line_length > 0 {
                        formatted_line.push(' ');
                        current_line_length += 1;
                    }

                    formatted_line.push_str(word);
                    current_line_length += word_length;
                }

                formatted_line
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}

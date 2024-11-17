use anyhow::Result;
use chrono::prelude::*;
use libsql::params;
use libsql::{Builder, Connection};
use serde_json::json;
use std::path::Path;
use tokio::fs;

pub async fn register_prompt(prompt: &str, answer: &str) -> Result<()> {
    match init_database("chat.db").await {
        Ok(connection) => {
            insert_db(&connection, prompt, answer).await;
            Ok(())
        }
        Err(e) => {
            eprintln!("Erro ao inicializar o banco de dados: {}", e);
            Err(e.into())
        }
    }
}

pub async fn history() -> Result<Vec<serde_json::Value>> {
    let mut chat_history = Vec::new();

    match init_database("chat.db").await {
        Ok(connection) => match query_db(&connection).await {
            Ok(results) => {
                for (query, answer) in results {
                    chat_history.push(json!({
                        "role": "user",
                        "content": query.as_str()
                    }));

                    chat_history.push(json!({
                        "role": "assistant",
                        "content": answer.as_str()
                    }));
                }
            }
            Err(e) => {
                eprintln!("Erro na query: {}", e);
                return Err(e.into());
            }
        },
        Err(e) => {
            eprintln!("Erro ao inicializar o banco de dados: {}", e);
            return Err(e.into());
        }
    }
    Ok(chat_history)
}

async fn query_db(conn: &Connection) -> Result<Vec<(String, String)>> {
    let mut rows = conn
        .query(
            "SELECT query, answer FROM chat ORDER BY created_at DESC LIMIT 7",
            (),
        )
        .await?;
    let mut results = Vec::new();
    while let Some(row) = rows.next().await? {
        let query: String = row.get(0).unwrap(); // Assuming column 0 is 'query'
        let answer: String = row.get(1).unwrap(); // Assuming column 1 is 'answer'
        results.push((query, answer));
    }
    Ok(results)
}

async fn insert_db(conn: &Connection, prompt: &str, answer: &str) -> () {
    let utc: DateTime<Utc> = Utc::now();
    let utc_str = utc.to_rfc3339();
    conn.execute(
        "INSERT INTO chat (id, query, answer, created_at) VALUES (1, ?1, ?2, ?3)",
        params![prompt, answer, utc_str],
    )
    .await
    .unwrap();
}

pub async fn init_database(db_path: &str) -> Result<Connection> {
    if !Path::new(db_path).exists() {
        println!("Banco de dados não encontrado. Criando...\n\n");
        create_database(db_path).await?;
    }
    let db = Builder::new_local(db_path).build().await?;
    let connection = db.connect()?;
    Ok(connection)
}

async fn create_database(db_path: &str) -> Result<()> {
    fs::File::create(db_path).await?;

    let db = Builder::new_local(db_path).build().await?;
    let conn = db.connect()?;

    conn.execute(
        "CREATE TABLE chat (
            id TEXT,
            query TEXT NOT NULL,
            answer TEXT NOT NULL,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            response_type TEXT,
            function TEXT,
            embed VECTOR[1536]
        );",
        (),
    )
    .await?;

    println!("Banco de dados e tabelas criados com sucesso.\n\n");
    Ok(())
}

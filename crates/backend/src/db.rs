use tokio_postgres::{NoTls, Client};
use std::env;
use crate::router::AppState;

pub async fn connect_db() -> Result<Client, tokio_postgres::Error> {
    let db_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "host=127.0.0.1 port=5432 user=jules dbname=codeza".to_string());
    let (client, connection) = tokio_postgres::connect(&db_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}

pub async fn init_postgres_db(state: &AppState) {
    let client = match connect_db().await {
        Ok(c) => c,
        Err(e) => {
            println!("PostgreSQL not available or connection failed ({}), skipping DB sync...", e);
            return;
        }
    };

    println!("Successfully connected to PostgreSQL database!");

    // Create tables
    let _ = client.batch_execute("
        CREATE TABLE IF NOT EXISTS users (
            id BIGINT PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            email TEXT
        );

        CREATE TABLE IF NOT EXISTS repos (
            id BIGINT PRIMARY KEY,
            name TEXT NOT NULL,
            owner TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS issues (
            id BIGINT PRIMARY KEY,
            repo_id BIGINT NOT NULL,
            number BIGINT NOT NULL,
            title TEXT NOT NULL,
            body TEXT,
            state TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS security_scans (
            id TEXT PRIMARY KEY,
            repo_owner TEXT NOT NULL,
            repo_name TEXT NOT NULL,
            timestamp TEXT NOT NULL,
            score INT NOT NULL,
            summary TEXT NOT NULL,
            details_json TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS action_run_logs (
            id BIGINT PRIMARY KEY,
            repo_id BIGINT NOT NULL,
            workflow_id BIGINT NOT NULL,
            name TEXT NOT NULL,
            status TEXT NOT NULL,
            conclusion TEXT NOT NULL,
            logs_json TEXT NOT NULL,
            created_at TEXT NOT NULL
        );
    ").await;

    // Load users from DB if existing
    let rows = client.query("SELECT id, username, email FROM users", &[]).await.unwrap_or_default();
    if !rows.is_empty() {
        let mut users_guard = state.users.write().unwrap();
        users_guard.clear();
        for row in rows {
            let id: i64 = row.get(0);
            let username: String = row.get(1);
            let email: Option<String> = row.get(2);
            users_guard.push(shared::User::new(id as u64, username, email));
        }
    } else {
        // Seed current users into Postgres
        let current_users = {
            let users_guard = state.users.read().unwrap();
            users_guard.clone()
        };
        for u in current_users {
            let _ = client.execute(
                "INSERT INTO users (id, username, email) VALUES ($1, $2, $3) ON CONFLICT (id) DO NOTHING",
                &[&(u.id as i64), &u.username, &u.email],
            ).await;
        }
    }

    // Load repos from DB if existing
    let repo_rows = client.query("SELECT id, name, owner FROM repos", &[]).await.unwrap_or_default();
    if !repo_rows.is_empty() {
        let mut repos_guard = state.repos.write().unwrap();
        repos_guard.clear();
        for row in repo_rows {
            let id: i64 = row.get(0);
            let name: String = row.get(1);
            let owner: String = row.get(2);
            repos_guard.push(shared::Repository::new(id as u64, name, owner));
        }
    } else {
        let current_repos = {
            let repos_guard = state.repos.read().unwrap();
            repos_guard.clone()
        };
        for r in current_repos {
            let _ = client.execute(
                "INSERT INTO repos (id, name, owner) VALUES ($1, $2, $3) ON CONFLICT (id) DO NOTHING",
                &[&(r.id as i64), &r.name, &r.owner],
            ).await;
        }
    }
}

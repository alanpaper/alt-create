use anyhow::{Error, Ok};

use sqlx::{query, query_as, Pool, Sqlite, SqlitePool};

#[derive(Debug, sqlx::FromRow)]
struct User {
    id: i32,
    name: String,
    email: String,
}

struct ConnectDb {
    pub pool: Pool<Sqlite>,
}

impl ConnectDb {
    pub async fn new(url: &str) -> ConnectDb {
        let pool = SqlitePool::connect(url)
            .await
            .expect("Couldn't connect to sqlite database");
        ConnectDb { pool }
    }

    async fn table_exists(&self, table_name: &str) -> Result<bool, Error> {
        let query = format!(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='{}'",
            table_name
        );
        let row = sqlx::query(&query).fetch_optional(&self.pool).await?;
        Ok(row.is_some())
    }

    async fn create_table(&mut self, table_name: String) {
        let sql = format!(
            "CREATE TABLE {} (
              id INTEGER PRIMARY KEY,
              name TEXT NOT NULL,
              email TEXT NOT NULL UNIQUE
            )",
            table_name
        );
        query(sql.as_str())
            .execute(&self.pool)
            .await
            .expect("CREATE TABLE notebook error");
    }

    async fn insert_table(&mut self, table_name: String, user: &User) {
        let sql = format!("INSERT INTO {} (name, email) VALUES (?, ?)", table_name);
        query(sql.as_str())
            .bind(user.name.clone())
            .bind(user.email.clone())
            .execute(&self.pool)
            .await
            .expect("INSERT table error");
    }

    async fn query_users(&self) -> Vec<User> {
        let users = query_as::<_, User>("SELECT * FROM users")
            .fetch_all(&self.pool)
            .await
            .expect("query data error");
        users
    }
}

pub async fn open_db() -> Result<(), anyhow::Error> {
    let mut content_db = ConnectDb::new("sqlite://mydatabase.db").await;
    let table_name = "notebook";
    let exists = content_db.table_exists(table_name).await?;
    if exists {
    } else {
        content_db.create_table(table_name.to_string()).await;
        content_db
            .insert_table(
                table_name.to_string(),
                &User {
                    id: 1,
                    name: String::from("test"),
                    email: String::from("123@qq.com"),
                },
            )
            .await;
    }

    let users = content_db.query_users().await;

    Ok(())
}

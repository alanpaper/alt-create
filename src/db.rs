
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions, query, query_as};

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub authorization: String,
}

pub struct Database {
    pub pool: SqlitePool,
}

impl Database {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let database_url = "db/mydatabase.db";
        let options = SqliteConnectOptions::new().filename(database_url).create_if_missing(true) ;
        let pool = SqlitePool::connect_with(options).await?;
        Ok(Self { pool })
    }

    pub async fn create_tables(&self) -> Result<(), sqlx::Error> {
        query("CREATE TABLE user (id INTEGER PRIMARY KEY, authorization TEXT NOT NULL)")
            .execute(&self.pool)
            .await?;
            Ok(())
    }

    pub async fn insert_user(&self, authorization: &str) -> Result<(), sqlx::Error> {
        query("INSERT INTO user (authorization) VALUES (?)")
            .bind(authorization)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_user(&self) -> Result<User, sqlx::Error> {
        let row = query_as::<_, User>("SELECT * FROM user")
            .fetch_one(&self.pool)
            .await?;
        Ok(row)
    }

}


pub async fn init_db(authorization: String) -> Result<(), sqlx::Error> {
    let db = Database::new().await?;
    let _ = db.create_tables().await?;
    let user = db.get_user().await;
    if let Err(_) = &user {
        let insert = db.insert_user(&authorization).await;
        if let Err(e) = insert {
            println!("Error inserting user: {}", e);
        }
    }
    Ok(())
}



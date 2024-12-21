use super::{Result, Store};
use crate::{
    auth::User,
    bank::{Account, Transaction},
};
use async_trait::async_trait;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

pub struct StoreSqlite {
    pool: Pool<Sqlite>,
}

impl StoreSqlite {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await
            .map_err(|_| super::Error::FailedToCreate)?;

        Ok(Self { pool })
    }
}

#[async_trait]
impl Store for StoreSqlite {
    async fn create_user(&self, name: &str, password_hash: &str) -> Result<User> {
        sqlx::query!(
            r#"
            INSERT INTO users (name, password_hash)
            VALUES (?, ?)
            "#,
            name,
            password_hash
        )
        .execute(&self.pool)
        .await
        .map_err(|_| super::Error::FailedToCreate)?;

        let record = sqlx::query!(
            r#"
            SELECT id, name, password_hash
            FROM users
            WHERE rowid = last_insert_rowid()
            "#
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|_| super::Error::FailedToCreate)?;

        Ok(User::new(
            record.id as usize,
            &record.name,
            &record.password_hash,
        ))
    }

    async fn get_user(&self, id: usize) -> Result<User> {
        todo!();
    }

    async fn create_account(&self, user_id: usize) -> Result<Account> {
        todo!()
    }

    async fn update_account(&self, account: Account) -> Result<Account> {
        todo!();
    }

    async fn get_account(&self, id: usize) -> Result<Account> {
        todo!();
    }
}

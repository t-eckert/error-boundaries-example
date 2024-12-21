use crate::{
    auth::User,
    bank::{Account, Transaction},
};

use super::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Store: Send + Sync {
    async fn create_user(&self, name: &str, password: &str) -> Result<User>;
    async fn get_user(&self, id: usize) -> Result<User>;
    async fn create_account(&self, user_id: usize) -> Result<Account>;
    async fn update_account(&self, account: Account) -> Result<Account>;
    async fn get_account(&self, id: usize) -> Result<Account>;
}

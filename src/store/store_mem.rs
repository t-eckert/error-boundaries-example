use super::{Error, Result, Store};
use crate::{
    auth::User,
    bank::{Account, Transaction},
};
use async_trait::async_trait;
use std::collections::HashMap;
use tokio::sync::Mutex;

pub struct StoreMem {
    users: Mutex<HashMap<usize, User>>,
    accounts: Mutex<HashMap<usize, Account>>,
    transactions: Mutex<HashMap<usize, Transaction>>,
}

impl StoreMem {
    pub fn new() -> Self {
        Self {
            users: Mutex::new(HashMap::new()),
            accounts: Mutex::new(HashMap::new()),
            transactions: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl Store for StoreMem {
    async fn create_user(&self, name: &str, password_hash: &str) -> Result<User> {
        let mut users = self.users.lock().await;

        let id = users.len() + 1;
        let user = User::new(id, name, password_hash);
        users.insert(id, user.clone());

        Ok(user)
    }

    async fn get_user(&self, id: usize) -> Result<User> {
        todo!();
    }

    async fn create_account(&self, user_id: usize) -> Result<Account> {
        let users = self.users.lock().await;
        let mut accounts = self.accounts.lock().await;

        let user = users
            .get(&user_id)
            .ok_or(Error::not_found(&format!("User with id {}", user_id)))?;

        let id = accounts.len() + 1;
        let account = Account::new(id, &user);

        accounts.insert(id, account.clone());

        Ok(account)
    }

    async fn update_account(&self, account: Account) -> Result<Account> {
        let mut accounts = self.accounts.lock().await;

        if accounts.contains_key(&account.id) {
            accounts.insert(account.id, account.clone());
        } else {
            return Err(Error::not_found(&format!("Account with id {}", account.id)));
        }

        Ok(account)
    }

    async fn get_account(&self, account_id: usize) -> Result<Account> {
        let accounts = self.accounts.lock().await;

        accounts
            .get(&account_id)
            .cloned()
            .ok_or(Error::not_found(&format!("Account with id {}", account_id)))
    }
}

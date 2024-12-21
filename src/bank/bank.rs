use super::{transaction, Account};
use super::{Error, Result};
use crate::auth::password;
use crate::{auth::User, store::Store};
use std::{collections::HashMap, sync::Arc};

#[derive(Clone)]
pub struct Bank {
    store: Arc<dyn Store + Send + Sync>,
}

impl Bank {
    pub fn new(store: Arc<dyn Store + Send + Sync>) -> Self {
        Self { store }
    }

    pub fn asdf() {
        println!("asdf")
    }

    pub async fn sign_up_new_user(&self, name: &str, password: &str) -> Result<User> {
        password::check_password_requirements(password)?;
        let password_hash = password::hash_password(password)?;

        Ok(self.store.create_user(name, &password_hash).await?)
    }

    pub async fn create_new_account_for_user(&self, user_id: usize) -> Result<Account> {
        let account_id = self.store.create_account(user_id).await?;

        Ok(account_id)
    }

    pub async fn get_account(&self, account_id: usize) -> Result<Account> {
        Ok(self.store.get_account(account_id).await?)
    }

    pub async fn process_transaction(
        &self,
        account_id: usize,
        action: transaction::Action,
        amount: u32,
    ) -> Result<()> {
        // There is 100% a race condition here. But this is just an example app.
        let mut account = self.store.get_account(account_id).await?;

        match action {
            transaction::Action::Deposit => {
                account.deposit(amount);
            }
            transaction::Action::Withdraw => {
                account.withdraw(amount)?;
            }
        }

        self.store.update_account(account).await?;

        Ok(())
    }
}

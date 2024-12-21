use crate::auth::User;

use super::{Error, Result};

#[derive(Debug, Clone)]
pub struct Account {
    pub id: usize,
    pub owner_id: usize,
    pub balance: u32,
}

impl Account {
    pub fn new(id: usize, user: &User) -> Self {
        Self {
            id,
            owner_id: user.id,
            balance: 0,
        }
    }

    pub fn deposit(&mut self, amount: u32) -> u32 {
        self.balance += amount;
        self.balance
    }

    pub fn withdraw(&mut self, amount: u32) -> Result<u32> {
        match amount <= self.balance {
            true => {
                self.balance -= amount;
                return Ok(self.balance);
            }
            false => Err(Error::AccountHasInsufficientBalance),
        }
    }
}

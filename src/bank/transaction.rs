pub struct Transaction {
    user_id: usize,
    account_id: usize,
    action: Action,
    amount: u32,
}

pub enum Action {
    Withdraw,
    Deposit,
}

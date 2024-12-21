#[derive(Clone)]
pub struct User {
    pub id: usize,
    pub name: String,
    password_hash: String,
}

impl User {
    pub fn new(id: usize, name: &str, password_hash: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            password_hash: password_hash.to_string(),
        }
    }
}

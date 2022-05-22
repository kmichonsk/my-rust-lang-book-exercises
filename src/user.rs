#[derive(Debug)]
pub struct User {
    pub email: String,
    pub username: String,
    pub active: bool,
    pub sign_in_count: u64,
}

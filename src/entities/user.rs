#[derive(Default, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
    pub campus_name: String,
    pub uid: u64,
    pub unid: u64,
    pub token: String,
}

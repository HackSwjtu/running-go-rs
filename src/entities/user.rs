#[derive(Default, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
    pub campus_name: String,
    pub uid: u32,
    pub unid: u32,
    pub token: String,
}

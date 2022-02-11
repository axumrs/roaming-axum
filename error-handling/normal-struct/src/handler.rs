use crate::extract::Json;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub email: String,
}
pub async fn login(Json(user): Json<User>) {
    dbg!(&user);
}

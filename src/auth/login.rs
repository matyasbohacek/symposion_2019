use rocket::request::FromForm;

#[derive(Debug, FromForm, Clone)]
pub struct Login {
    pub login: String,
    pub password: String,
}

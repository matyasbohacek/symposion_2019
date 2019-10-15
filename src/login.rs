use rocket::request::FromForm;

#[derive(Debug, FromForm)]
pub struct Login {
    pub login: String,
    pub password: String,
}

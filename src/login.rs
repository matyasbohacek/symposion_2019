#[cfg(feature = "backend")]
use rocket::request::FromForm;

#[cfg_attr(feature = "backend", derive(Debug, FromForm, Clone))]
#[cfg_attr(feature = "frontend", derive(Debug, Clone))]
pub struct Login {
    pub login: String,
    pub password: String,
}

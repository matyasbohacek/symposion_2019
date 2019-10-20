use rocket::request::{self, FromRequest, Request};
use rocket::http::{Cookie, Cookies, Status};

pub struct AdminGuard(String);

impl<'a, 'r> FromRequest<'a, 'r> for AdminGuard{
    type Error = String;
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error>{
        let login = request.cookies()
            .get_private("login")
            .unwrap_or(Cookie::new("login", "none"))
            .value();
        request::Outcome::Success(AdminGuard(login.to_string()))
    }
}

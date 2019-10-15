
use rocket::request::{FromRequest, Request};
use rocket::Outcome;
use rocket::http::{Cookie, Cookies};

pub struct AdminGuard;

impl<'a, 'r> FromRequest<'a, 'r> for AdminGuard{
    type Error = !;
    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error>{
        request.cookies()
            .get_private("admin")
            .map(|&x| {println!("{}", x)});
        Outcome::Success()
    }
}


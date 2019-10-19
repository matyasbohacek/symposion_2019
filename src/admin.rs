
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use rocket::http::{Cookie, Cookies};

pub struct AdminGuard;


impl<'a, 'r> FromRequest<'a, 'r> for AdminGuard{
    type Error = String;
    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error>{
        request.cookies()
            .get_private("login")
            .map(|&x| {println!("{}", x)});
        Outcome::Success()
    }
}

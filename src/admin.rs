
use rocket::request::FromRequest;

pub struct AdminGuard;

impl FromRequest for AdminGuard{
    type Error = String;
    fn from_request(){
        unimplemented!();
    }
}


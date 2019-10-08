#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::NamedFile;
use rocket::data::FromDataSimple;
use rocket::{Request, Data, Outcome::*};
use rocket::data::Outcome;
use rocket::http::{Cookies, Cookie, ContentType, Status};
use std::io::Read;

const LIMIT: u64 = 256; // input data limit

struct Login{
    login: String,
    password: String,
}

impl FromDataSimple for Login{
    type Error = String;

    fn from_data(req: &Request, data: Data) -> Outcome<Self, String>{
        let person_ct = ContentType::new("application", "x-login");
        if req.content_type() != Some(&person_ct) {
            return Outcome::Forward(data);
        }

        let mut string = String::new();
        if let Err(e) = data.open().take(LIMIT).read_to_string(&mut string) {
            return Failure((Status::InternalServerError, format!("{:?}", e)));
        }

        let (login, password) = match string.find(':') {
            Some(i) => (string[..i].to_string(), &string[(i + 1)..]),
            None => return Failure((Status::UnprocessableEntity, "':'".into()))
        };

        Success(Login{
            login: login.to_string(),
            password: password.to_string(),
        })

    }
}

//struct AdminGuard;

//impl FromRequest for AdminGuard{
    //type Error = String;
    //fn from_request(){
        //unimplemented!();
    //}
//}


//#[get("/admin")]
//fn admin(admin: AdminGuard){
    //unimplemented!();
//}

#[post("/login", data="<logindata>")]
fn login_post(logindata: Login, mut cookies: Cookies){
    cookies.add_private(Cookie::new("admin", "true")) // TODO
}

#[get("/login")]
fn login(){
    unimplemented!();
}

#[get("/")]
fn index() -> NamedFile {
    NamedFile::open("conf/index.html").expect("FUCK")
}



fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}


#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::NamedFile;
use rocket::data::FromDataSimple;
use rocket::{Request, Data};
use rocket::data::Outcome;
use toml::Value;

struct Login{
    login: String,
    password: String,
}

impl FromDataSimple for Login{
    type Error = String;

    fn from_data(req: &Request, data: Data) -> Outcome<Self, String>{
        unimplemented!();
        
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
fn login_post(logindata: Login){
    unimplemented!();
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


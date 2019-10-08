#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::request::{FromRequest, Form};

#[derive(FromForm)]
struct Login{
    login: String,
    password: String,
}

struct AdminGuard;

impl FromRequest for AdminGuard{
    fn 
}


#[get("/admin")]
fn admin(admin: AdminGuard){
    unimplemented!();
}

#[post("/login", data="<logindata>")]
fn login_post(logindata:){
    unimplemented!();
}

#[get("/login")]
fn login(){
    unimplemented!();
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}



fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}


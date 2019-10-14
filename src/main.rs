#![feature(proc_macro_hygiene, decl_macro)]

// macros
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

// rocket imports
use rocket::http::{Cookie, Cookies};
use rocket::response::NamedFile;

//modules
mod login;
mod schema;
mod db;

use db::*;
use login::*;

//#[get("/admin")]
//fn admin(admin: AdminGuard){
    //unimplemented!();
//}

//#[post("/login", data = "<logindata>")]
//fn login_post(logindata: Login, db: Users, mut cookies: Cookies){
    //cookies.add_private(Cookie::new("admin", "true")); // TODO
//}

//#[get("/login")]
//fn login(db: Users) -> String {
//}

#[get("/")]
fn index() -> NamedFile {
    NamedFile::open("www/index.html").expect("FUCK")
}

#[get("/style/<file>")]
fn styling(file: String) -> Option<NamedFile>{
    NamedFile::open(format!("style/{}", file)).ok()
}

fn main() {
    rocket::ignite()
        .attach(Users::fairing())
        .mount("/", routes![index, styling/*, login*/])
        .launch();
}

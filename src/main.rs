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
use diesel::prelude::*;

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
    //use schema::users::dsl::*;
    //let result = users.filter(login.eq("admin2"))
        //.load::<User>(&*db)
        //.expect("oops");
    //cookies.add_private(Cookie::new("admin", "true")); // TODO
//}

#[get("/login")]
fn login() -> NamedFile {
    NamedFile::open("www/login.html").unwrap()
}

#[get("/")]
fn index() -> NamedFile {
    NamedFile::open("www/index.html").unwrap()
}

#[get("/style/<file>")]
fn styling(file: String) -> Option<NamedFile>{
    NamedFile::open(format!("style/{}", file)).ok()
}

fn main() {
    rocket::ignite()
        .attach(Users::fairing())
        .mount("/", routes![index, styling, login])
        .launch();
}

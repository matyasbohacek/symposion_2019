#![feature(proc_macro_hygiene, decl_macro)]

// macros
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

// rocket imports
use diesel::prelude::*;
use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::{NamedFile, Redirect};

//modules
mod db;
mod login;
mod schema;
mod admin;

use db::*;
use login::*;
use admin::*;

#[get("/admin")]
fn admin(admin: AdminGuard) -> String {
    //TODO
    "congrats".to_string()
}

#[post("/login", data = "<logindata>")]
fn login_post(logindata: Form<Login>, db: Users, mut cookies: Cookies) -> Redirect {
    use schema::users::dsl::*;
    let result = users
        .filter(login.eq(&logindata.login))
        .filter(password.eq(&logindata.password))
        .load::<User>(&*db)
        .unwrap();

    if result.len() > 0 {
        cookies.add_private(Cookie::new("admin", "true"));
        return Redirect::to(uri!(admin));
    } else {
        return Redirect::to(uri!(login));
    }
}

#[get("/login")]
fn login() -> NamedFile {
    NamedFile::open("www/login.html").unwrap()
}

#[get("/")]
fn index() -> NamedFile {
    NamedFile::open("www/index.html").unwrap()
}

#[get("/style/<file>")]
fn styling(file: String) -> Option<NamedFile> {
    NamedFile::open(format!("style/{}", file)).ok()
}

#[catch(404)]
fn not_found() -> NamedFile {
    NamedFile::open("www/404.html").unwrap()
}

fn main() {
    rocket::ignite()
        .register(catchers![not_found])
        .attach(Users::fairing())
        .mount("/", routes![index, styling, login, login_post, admin])
        .launch();
}

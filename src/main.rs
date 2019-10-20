#![feature(proc_macro_hygiene, decl_macro)]

// macros
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;

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

#[get("/admin", data="<login>")]
fn admin(admin: AdminGuard, login: String) -> String {
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
        cookies.add_private(Cookie::new("login", logindata.clone().login));
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

#[get("/static/<file>")]
fn static_files(file: String) -> Option<NamedFile> {
    NamedFile::open(format!("static/{}", file)).ok()
}

#[get("/pkg/<file>")]
fn wasm_styling(file: String) -> Option<NamedFile> {
    NamedFile::open(format!("pkg/{}", file)).ok()
}

#[catch(404)]
fn not_found() -> NamedFile {
    NamedFile::open("www/404.html").unwrap()
}

fn main() {
    rocket::ignite()
        .register(catchers![not_found])
        .attach(Users::fairing())
        .mount("/", routes![index, styling, static_files, wasm_styling, login, login_post, admin])
        .launch();
}

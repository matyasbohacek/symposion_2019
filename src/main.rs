#![cfg_attr(feature = "backend", feature(proc_macro_hygiene, decl_macro))]

// macros
#[cfg_attr(feature = "backend", macro_use)]
#[cfg(feature = "backend")]
extern crate rocket;
#[cfg(feature = "backend")]
#[macro_use]
extern crate rocket_contrib;
#[cfg(feature = "backend")]
#[macro_use]
extern crate diesel;

// rocket imports
#[cfg(feature = "backend")] use diesel::prelude::*;
#[cfg(feature = "backend")] use rocket::http::{Cookie, Cookies};
#[cfg(feature = "backend")] use rocket::request::Form;
#[cfg(feature = "backend")] use rocket::response::{NamedFile, Redirect};

//modules
#[cfg(feature = "backend")] mod db;
mod login;
#[cfg(feature = "backend")] mod schema;
//mod admin;

#[cfg(feature = "backend")] use db::*;
use login::*;
//use admin::*;

#[cfg(feature = "backend")]
#[get("/admin")]
fn admin(/*admin: AdminGuard*/) -> String {
    //TODO
    "congrats".to_string()
}

#[cfg(feature = "backend")]
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

#[cfg(feature = "backend")]
#[get("/login")]
fn login() -> NamedFile {
    NamedFile::open("www/login.html").unwrap()
}

#[cfg(feature = "backend")]
#[get("/")]
fn index() -> NamedFile {
    NamedFile::open("www/index.html").unwrap()
}

#[cfg(feature = "backend")]
#[get("/style/<file>")]
fn styling(file: String) -> Option<NamedFile> {
    NamedFile::open(format!("style/{}", file)).ok()
}

#[cfg(feature = "backend")]
#[catch(404)]
fn not_found() -> NamedFile {
    NamedFile::open("www/404.html").unwrap()
}

fn main() {
    if cfg!(feature = "backend") { println!("backend") }
    if cfg!(feature = "frotnend") { println!("frontend") }

    #[cfg(feature = "backend")] {
        rocket::ignite()
            .register(catchers![not_found])
            .attach(Users::fairing())
            .mount("/", routes![index, styling, login, login_post, admin])
            .launch();
        println!("yes homo");
    }

    #[cfg(feature = "frontend")] {
        println!("no homo");
    }
}

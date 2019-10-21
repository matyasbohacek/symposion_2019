mod login;
mod admin;

use crate::{
	db::*,
	auth::login::*,
	auth::admin::*,
};

// rocket imports
use diesel::prelude::*;
use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::{NamedFile, Redirect};

#[get("/admin?<login>")]
pub(crate) fn admin(admin: AdminGuard, login: String) -> String {
    if admin.0 == login{
        "congrats".to_string()
    } else{
        "fuck off".to_string()
    }
}

#[post("/login", data = "<logindata>")]
pub(crate) fn login_post(logindata: Form<Login>, db: Users, mut cookies: Cookies) -> Redirect {
    use crate::schema::users::dsl::*;
    let result = users
        .filter(login.eq(&logindata.login))
        .filter(password.eq(&logindata.password))
        .load::<User>(&*db)
        .unwrap();

    if result.len() > 0 {
        cookies.add_private(Cookie::new("login", logindata.login.clone()));
        return Redirect::to(uri!(admin: logindata.login.clone()));
    } else {
        return Redirect::to(uri!(login));
    }
}

#[get("/login")]
pub(crate) fn login() -> NamedFile {
	NamedFile::open("www/login.html").unwrap()
}

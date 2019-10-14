#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket::data::FromDataSimple;
use rocket::data::Outcome;
use rocket::http::{ContentType, Cookie, Cookies, Status};
use rocket::response::NamedFile;
use rocket::{Data, Outcome::*, Request};
use std::io::Read;

use rocket_contrib::databases::diesel::sqlite::{SqliteConnection, SqliteQueryBuilder};
use rocket_contrib::databases::diesel::Connection;
use rocket_contrib::databases::diesel::connection::SimpleConnection;
use rocket_contrib::databases::diesel::query_builder::QueryBuilder;

const LIMIT: u64 = 256; // input data limit

struct Login {
    login: String,
    password: String,
}

impl FromDataSimple for Login {
    type Error = String;

    fn from_data(req: &Request, data: Data) -> Outcome<Self, String> {
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
            None => return Failure((Status::UnprocessableEntity, "':'".into())),
        };

        Success(Login {
            login: login.to_string(),
            password: password.to_string(),
        })
    }
}

#[database("sqlite_users")]
struct Users(SqliteConnection);

fn getuser(conn: &SqliteConnection) -> bool {
    true //TODO
}

/*
struct AdminGuard;

impl FromRequest for AdminGuard{
    type Error = String;
    fn from_request(){
        unimplemented!();
    }
}


#[get("/admin")]
fn admin(admin: AdminGuard){
    unimplemented!();
}
*/

#[post("/login", data = "<logindata>")]
fn login_post(logindata: Login, db: Users, mut cookies: Cookies) -> String {
    cookies.add_private(Cookie::new("admin", "true")) // TODO
}

#[get("/login")]
fn login(db: Users) -> String {
    format!("{}", getuser(&*db)) // zatim debug stranka
        // zapasim totiz s databazi
}

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
        .mount("/", routes![index, styling, login])
        .launch();
}

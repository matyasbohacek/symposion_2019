#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/admin")]
fn admin(){
    unimplemented!();
}

#[post("/login")]
fn login_post(){
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


#![feature(proc_macro_hygiene, decl_macro)]

// macros
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

//modules
mod auth;
mod db;
mod schema;
mod static_server;

fn main() {
    rocket::ignite()
        .register(catchers![static_server::not_found])
        .attach(db::Users::fairing())
        .mount(
            "/",
            routes![
                static_server::index,
                static_server::styling,
                static_server::static_files,
                static_server::wasm_styling,
                auth::login,
                auth::login_post,
                auth::admin
            ],
        )
        .launch();
}

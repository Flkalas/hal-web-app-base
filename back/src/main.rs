#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::serve::StaticFiles;
use rocket::response::{NamedFile};

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open("/index.html").ok()
}

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from("../front/dist"))
        .launch();
}

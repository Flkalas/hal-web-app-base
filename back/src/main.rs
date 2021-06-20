#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket::response::NamedFile;
use rocket_contrib::databases::diesel;
use rocket_contrib::serve::StaticFiles;

#[database("diesel_sqlite_pool")]
struct DatabaseConnection(diesel::SqliteConnection);

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open("/index.html").ok()
}

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from("../front/dist"))
        .launch();
}

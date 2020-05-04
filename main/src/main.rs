#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use function;

fn main() {
    rocket::ignite().mount("/", routes![function::function]).launch();
}
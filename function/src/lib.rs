#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
pub fn function() -> &'static str {
    "Hello, world!"
}
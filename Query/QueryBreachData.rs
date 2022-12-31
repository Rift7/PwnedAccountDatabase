extern crate actix_web;
extern crate sled;

use actix_web::{web, App, HttpServer, Responder};

fn index(info: web::Path<(String,)>) -> impl Responder {
    let db = sled::open("emails_and_passwords.db").unwrap();

    let query = info.0.as_bytes();
    let mut response = String::new();

    for result in db.scan_prefix(query) {
        let (key, value) = result.unwrap();
        let email = String::from_utf8(key).unwrap();
        let (password, third_variable) = bincode::deserialize(&value).unwrap();

        response.push_str(&format!("Email: {} Password: {} Third variable: {}<br>", email, password, third_variable));
    }

    format!("{}", response)
}

fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/{query}", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
}

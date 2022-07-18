#[macro_use]
extern crate rocket;


mod routes {
    pub mod index;
    pub mod contacts;
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/", 
            routes![
                routes::index::index, 
                routes::contacts::contacts,
                ]
            )
}

#[macro_use]
extern crate rocket;


mod routes {
    pub mod index;
    pub mod contacts;
}

pub mod db;

#[launch]
fn rocket() -> _ {
    let _connection = db::get_connection();

    rocket::build()
        .mount(
            "/", 
            routes![
                routes::index::index, 
                routes::contacts::contacts,
                ]
            )
}

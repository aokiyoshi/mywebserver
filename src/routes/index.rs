use askama::Template; 
use rocket::response::content::RawHtml;

use crate::db::get_all_users;

#[derive(Template)] 
#[template(path = "../templates/index.html")]                              
struct IndexTemplate<'a> { 
    name: &'a str,
    users: String
}

#[get("/")]
pub fn index() -> RawHtml<String>{
    
    let index = IndexTemplate { name: "yeah! its working!", users: get_all_users() }; 
    RawHtml(index.render().unwrap())
}
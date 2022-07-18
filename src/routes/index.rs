use askama::Template; 
use rocket::response::content::RawHtml;

#[derive(Template)] 
#[template(path = "../templates/index.html")]                              
struct IndexTemplate<'a> { 
    name: &'a str 
}

#[get("/")]
pub fn index() -> RawHtml<String>{
    let index = IndexTemplate { name: "yeah! its working!" }; 
    RawHtml(index.render().unwrap())
}
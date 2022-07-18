use askama::Template; 
use rocket::response::content::RawHtml;

#[derive(Template)] 
#[template(path = "../templates/contacts.html")]                              
struct ContactsTemplate<'a> { 
    name: &'a str,
    email: &'a str,
    phone: &'a str
}

#[get("/contacts")]
pub fn contacts() -> RawHtml<String>{
    let cont = ContactsTemplate { 
        name: "yoshi",
        email: "yaoki92@gmail.com",
        phone: "0-000-000-000"
    }; 
    RawHtml(cont.render().unwrap())
}
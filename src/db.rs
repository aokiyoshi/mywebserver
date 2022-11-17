use sqlite::Connection;



pub(super) fn get_connection() -> Connection {
    sqlite::open("database.db").unwrap()   
}

pub fn get_all_users() -> String {

    let connection = get_connection();

    let query = "
        SELECT * from users;
    ";

    let mut users = vec![];

    connection
    .iterate(query, |pairs| {
        for &(name, value) in pairs.iter() {
            users.push(format!("{} = {}", name, value.unwrap()));
        }
        true
    })
    .unwrap();
    users.join(", ")

}
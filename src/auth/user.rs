pub struct User {
    pub name: String,
    pub phone: String,
    pub city: String,
    pub state: String,
}

pub fn get_user() -> User {
    User {
        name: String::from("Vaibhav"),
        phone: String::from("7738994562"),
        city: String::from("MUM"),
        state: String::from("MH"),
    }
}

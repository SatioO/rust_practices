mod auth;
mod todos;

use auth::AuthStatus;
use todos::todo::Todo;
use todos::Todos;

fn main() {
    let mut todos = Todos::new();

    todos.add(Todo::new(1, String::from("TODO-1")));
    todos.add(Todo::new(2, String::from("TODO-2")));
    todos.add(Todo::new(3, String::from("TODO-3")));
    todos.add(Todo::new(4, String::from("TODO-4")));

    todos.remove(2);

    let user = auth::login(auth::Credentials {
        username: String::from("admin"),
        password: String::from("admin"),
    });

    match user {
        Some(user) => println!("{}", user.name),
        None => println!("None"),
    }
}

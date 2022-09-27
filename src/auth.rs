pub mod user;

pub struct Credentials {
    pub username: String,
    pub password: String,
}

pub enum AuthStatus {
    Authenticated,
    UnAuthenticated,
}

pub fn login(creds: Credentials) -> Option<user::User> {
    let auth_status = AuthStatus::Authenticated;

    if matches!(auth_status, AuthStatus::Authenticated) {
        Some(user::get_user())
    } else {
        None
    }
}

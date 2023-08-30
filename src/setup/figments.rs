use rocket::figment::{providers::Serialized, Figment};

#[cfg(debug_assertions)]
pub fn get_figment() -> Figment {
    rocket::Config::figment()
        .merge(Serialized::defaults(rocket::Config::default()))
        .merge(("ident", "AntiAfkBackend"))
}

#[cfg(not(debug_assertions))]
pub fn get_figment() -> Figment {
    rocket::Config::figment()
        .merge(Serialized::defaults(rocket::Config::default()))
        .merge(("ident", "AntiAfkBackend"))
        .merge(("template_dir", "."))
}

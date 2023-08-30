use rocket::{Build, Rocket};

use rocket_dyn_templates::Template;

#[cfg(debug_assertions)]
pub fn setup_templates(rocket_build: Rocket<Build>) -> Rocket<Build> {
    rocket_build.attach(Template::fairing())
}

#[cfg(not(debug_assertions))]
pub fn setup_templates(rocket_build: Rocket<Build>) -> Rocket<Build> {
    rocket_build.attach(Template::custom(get_templates))
}

#[cfg(not(debug_assertions))]
fn get_templates(engines: &mut rocket_dyn_templates::Engines) {
    engines
        .tera
        .add_raw_templates([
            (
                "index",
                include_str!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/templates/index.html.tera"
                )),
            ),
            (
                "repeat-key",
                include_str!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/templates/repeat-key.html.tera"
                )),
            ),
        ])
        .expect("valid Tera templates");
}

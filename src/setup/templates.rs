macro_rules! template_location {
    ($filename:literal, $extention:ident) => {
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/templates/",
            $filename,
            ".",
            stringify!($extention),
            ".tera"
        )
    };
}

macro_rules! include_templates {
    ($($filename:literal)+, $extention:ident) => {[
        $((
            $filename,
            include_str!(template_location!($filename, $extention)),
        ),)*
    ]};
}

type RawTemplate = (&'static str, &'static str);
type RawTemplates = [RawTemplate; 2];

const RAW_TEMPLATES: RawTemplates = include_templates!("index" "repeat-key", html);

pub fn templates(engines: &mut rocket_dyn_templates::Engines) {
    if cfg!(debug_assertions) {
        return;
    }

    engines
        .tera
        .add_raw_templates(RAW_TEMPLATES)
        .expect("valid Tera templates");
}

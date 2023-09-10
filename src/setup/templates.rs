type RawTemplate<'a> = (&'a str, &'a str);

macro_rules! parse_extensions {
    ($filename:expr) => {{
        let parts: Vec<&str> = $filename.split('.').collect();

        &parts[0..parts.len() - 2].join(".")
    }};
}

macro_rules! include_template {
    ($filename:expr) => {
        (
            parse_extensions!($filename),
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/templates/",
                $filename
            )),
        )
    };
}
pub fn templates(engines: &mut rocket_dyn_templates::Engines) {
    if cfg!(debug_assertions) {
        return;
    }

    let raw_templates: [RawTemplate; 2] = [
        include_template!("index.html.tera"),
        include_template!("repeat-key.html.tera"),
    ];

    engines
        .tera
        .add_raw_templates(raw_templates)
        .expect("valid Tera templates");
}

fn main() {
    let profile = std::env::var("PROFILE").expect("profile");

    let output_dir = format!("target/{}/templates", profile);

    std::fs::create_dir_all(&output_dir).expect("create directory");
    // // Specify the source and destination directories

    // Copy the contents of the source directory to the target directory
    for entry in std::fs::read_dir("templates").expect("read templates") {
        let file = entry.expect("directory entry").path();
        let file_name = file.file_name().expect("file name").to_string_lossy();

        let output_path = format!("{}/{}", &output_dir, file_name);

        std::fs::copy(&file, output_path).expect("copy files");
    }
}

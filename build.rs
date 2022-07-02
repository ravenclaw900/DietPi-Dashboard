use std::io::Read;

fn main() {
    if std::env::var_os("CARGO_FEATURE_FRONTEND").is_some() {
        println!("cargo:rerun-if-changed=frontend/src/");
        println!("cargo:rerun-if-changed=frontend/package.json");

        let frontend_path = concat!(env!("CARGO_MANIFEST_DIR"), "/frontend");
        let dist_path = concat!(env!("CARGO_MANIFEST_DIR"), "/frontend/dist");

        std::process::Command::new("yarn")
            .arg("install")
            .current_dir(frontend_path)
            .output()
            .expect("Can't run yarn install");

        std::process::Command::new("yarn")
            .arg("build")
            .current_dir(frontend_path)
            .output()
            .expect("Can't run yarn build");

        if std::env::var("PROFILE").unwrap_or_default() == "release" {
            for i in walkdir::WalkDir::new(dist_path)
                .into_iter()
                .filter_entry(|e| e.path().extension().unwrap_or_default() != "png")
            {
                let entry = i.expect("Couldn't get file data");

                if !entry.file_type().is_file() {
                    continue;
                }

                let mut compressed = flate2::read::GzEncoder::new(
                    std::fs::File::open(entry.path()).expect("Couldn't open uncompressed file"),
                    flate2::Compression::best(),
                );
                let mut buf = Vec::new();
                compressed
                    .read_to_end(&mut buf)
                    .expect("Couldn't read compressed data");
                std::fs::write(entry.path(), &buf).expect("Couldn't write compressed data to file");
            }
        }
    }
}

use std::io::Write;

fn main() {
    if std::env::var_os("CARGO_FEATURE_FRONTEND").is_some() {
        let frontend_path = concat!(env!("CARGO_MANIFEST_DIR"), "/frontend");
        let dist_path = concat!(env!("CARGO_MANIFEST_DIR"), "/frontend/dist");
        let type_files = ["BackendData", "FrontendRequest"];

        println!(
            "{}",
            rerun_in_except::rerun_in_except(frontend_path, &[dist_path])
                .expect("Couldn't get frontend directory")
        );

        for i in type_files {
            std::process::Command::new("jtd-codegen")
                .args([
                    &format!("schemas/{i}.jtd.json"),
                    "--rust-out",
                    "src/types",
                    "--typescript-out",
                    "frontend/src/types",
                ])
                .output()
                .expect("Couldn't generate types from JTD");
            std::fs::rename("src/types/mod.rs", format!("src/types/{i}.rs"))
                .expect("Couldn't rename Rust type file");
            std::fs::rename(
                "frontend/src/types/index.ts",
                format!("frontend/src/types/{i}.ts"),
            )
            .expect("Couldn't rename Typescript type file");
        }

        std::fs::write("src/types/mod.rs", "").expect("Couldn't create types/mod.rs");

        for i in type_files {
            writeln!(
                std::fs::OpenOptions::new()
                    .append(true)
                    .open("src/types/mod.rs")
                    .expect("Couldn't open types/mod.rs for appending"),
                "mod {i};"
            )
            .expect("Couldn't write new module to file");
        }

        std::process::Command::new("sh")
            .args(["-c", "pnpm install"])
            .current_dir(frontend_path)
            .output()
            .expect("Can't run pnpm install");

        std::process::Command::new("sh")
            .args(["-c", "pnpm build"])
            .current_dir(frontend_path)
            .output()
            .expect("Can't run pnpm build");

        if std::env::var("PROFILE").unwrap_or_default() == "release" {
            for i in walkdir::WalkDir::new(dist_path)
                .into_iter()
                .filter_entry(|e| e.path().extension().unwrap_or_default() != "png")
            {
                let entry = i.expect("Couldn't get file data");

                if !entry.file_type().is_file() {
                    continue;
                }

                let buf = std::fs::read(entry.path()).expect("Couldn't get uncompressed data");
                let mut compressed = flate2::write::GzEncoder::new(
                    std::fs::File::create(entry.path()).expect("Couldn't open uncompressed file"),
                    flate2::Compression::best(),
                );
                compressed
                    .write_all(&buf)
                    .expect("Couldn't read compressed data");
                compressed
                    .finish()
                    .expect("Couldn't finish writing compressed data");
            }
        }
    }
}

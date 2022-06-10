use std::io::Read;

fn main() {
    for i in walkdir::WalkDir::new("dist")
        .into_iter()
        .filter_entry(|e| e.path().extension().unwrap_or_default() != "png")
    {
        let entry = i.expect("Couldn't get file data");

        if !entry.file_type().is_file() {
            continue;
        }

        let mut compressed = flate2::read::GzEncoder::new(
            std::fs::File::open(entry.path()).expect("Couldn't open uncompressed file"),
            flate2::Compression::new(5),
        );
        let mut buf = Vec::new();
        compressed
            .read_to_end(&mut buf)
            .expect("Couldn't read compressed data");
        std::fs::write(entry.path(), &buf).expect("Couldn't write compressed data to file");
    }
}

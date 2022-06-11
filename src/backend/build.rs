use std::io::{Read, Write};

fn main() {
    for i in walkdir::WalkDir::new("dist")
        .into_iter()
        .filter_entry(|e| e.path().extension().unwrap_or_default() != "png")
    {
        let entry = i.expect("Couldn't get file data");

        if !entry.file_type().is_file() {
            continue;
        }

        let mut buf = Vec::new();

        std::fs::File::open(entry.path())
            .expect("Couldn't open uncompressed file")
            .read_to_end(&mut buf)
            .expect("Couldn't read uncompressed file");

        let mut compressed = brotli::CompressorWriter::new(
            std::fs::File::create(entry.path()).expect("Couldn't init compressed file"),
            4096,
            11,
            22,
        );

        compressed
            .write_all(&buf)
            .expect("Couldn't read compressed data");
        compressed
            .flush()
            .expect("Couldn't write compressed data to file");
    }
}

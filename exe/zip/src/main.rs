use miniz_oxide::{deflate::compress_to_vec, inflate::decompress_to_vec};

fn roundtrip(data: &[u8]) {
    let compressed = compress_to_vec(data, 6);
    let decompressed = decompress_to_vec(compressed.as_slice()).unwrap();
    assert_eq!(data, decompressed);
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: {} reps filename", args[0]);
        return;
    }
    let reps = args[1].parse::<usize>().unwrap();
    let filename = &args[2];

    println!("reading {filename}...");
    let buf = std::fs::read(filename).unwrap();
    // let buf = filename.as_bytes();
    for _ in 0..reps {
        roundtrip(&buf);
    }
    println!("roundtripped {} bytes {reps} times", buf.len());
}

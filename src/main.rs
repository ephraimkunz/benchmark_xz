use std::io::BufReader;
use std::io::Read;

const FILENAME: &str = "data/3167.xz";

fn main() {
    println!(
        "Run \"cargo test --release\" to test that all the XZ libraries return the same result."
    );
    println!("Run \"cargo criterion\" to benchmark all the XZ libraries.");

    xz2_decompress();
}

fn xz2_decompress() -> Vec<u8> {
    let mut output = Vec::new();
    let r = BufReader::new(std::fs::File::open(FILENAME).expect("Couldn't open xz file"));
    xz2::bufread::XzDecoder::new(r)
        .read_to_end(&mut output)
        .expect("read to end");
    output
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use super::xz2_decompress;

    const FILENAME: &str = "data/3167.xz";

    fn lzma_rs_decompress() -> Vec<u8> {
        let mut output = Vec::new();
        let mut r = BufReader::new(std::fs::File::open(FILENAME).expect("Couldn't open xz file"));
        lzma_rs::xz_decompress(&mut r, &mut output).expect("xz_decompress in lzma_rs");
        output
    }

    fn xz_decom_decompress() -> Vec<u8> {
        let input = std::fs::read(FILENAME).unwrap();
        xz_decom::decompress(&input).unwrap()
    }

    fn rust_lzma_decompress() -> Vec<u8> {
        let input = std::fs::read(FILENAME).unwrap();
        lzma::decompress(&input).unwrap()
    }

    #[test]
    fn results_equal() {
        let reference = lzma_rs_decompress();
        assert_eq!(reference, xz2_decompress());
        assert_eq!(reference, xz_decom_decompress());
        assert_eq!(reference, rust_lzma_decompress());
    }
}

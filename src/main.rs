use std::io::BufReader;
use std::io::Read;

const FILENAME: &str = "3167.xz";

fn main() {
    // lzma_rs_decompress();
    xz2_decompress();
}

fn lzma_rs_decompress() -> Vec<u8> {
    let mut output = Vec::new();
    let mut r = BufReader::new(std::fs::File::open(FILENAME).expect("Couldn't open xz file"));
    lzma_rs::xz_decompress(&mut r, &mut output).expect("xz_decompress in lzma_rs");
    output
}

fn xz2_decompress() -> Vec<u8> {
    let mut output = Vec::new();
    let r = BufReader::new(std::fs::File::open(FILENAME).expect("Couldn't open xz file"));
    xz2::bufread::XzDecoder::new(r)
        .read_to_end(&mut output)
        .expect("read to end");
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

#[cfg(test)]
mod tests {
    use crate::{lzma_rs_decompress, rust_lzma_decompress, xz2_decompress, xz_decom_decompress};

    #[test]
    fn results_equal() {
        let reference = lzma_rs_decompress();
        assert_eq!(reference, xz2_decompress());
        assert_eq!(reference, xz_decom_decompress());
        assert_eq!(reference, rust_lzma_decompress());
    }
}

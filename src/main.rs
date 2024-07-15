use std::{fs::File, io::Read};

use bip39::Mnemonic;
use rustc_serialize::base64::{ToBase64, MIME};
use sha2::{Digest, Sha256};

pub fn to_base64(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut vec = vec![];
    file.read_to_end(&mut vec).unwrap();
    let base64 = vec.to_base64(MIME);
    base64.replace("\r\n", "")
}

fn main() {
    let base64 = to_base64("./input/image.jpg");
    let mut hasher = Sha256::new();
    hasher.update(base64);
    let entropy = hasher.finalize();
    let bip = Mnemonic::from_entropy(&entropy).unwrap();
    let result = bip.word_iter().fold(String::new(), |str1, str2| str1 + str2 + " ");
    println!("{}", result);
}

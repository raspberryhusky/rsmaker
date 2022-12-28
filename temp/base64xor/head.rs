#![windows_subsystem = "windows"]
use windows::Win32::System::Memory::VirtualAlloc;
use windows::Win32::System::Memory;
use windows::Win32::System::Threading::{CreateThread,THREAD_CREATION_FLAGS,WaitForSingleObject};
use windows::Win32::UI::WindowsAndMessaging::{ShowWindow,SHOW_WINDOW_CMD,SW_HIDE};
use windows::Win32::System::Console::GetConsoleWindow;
use windows::Win32::Globalization::GetSystemDefaultLangID;
use std::{env,ptr,mem,time,thread};
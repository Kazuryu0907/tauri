use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Gdi::ValidateRect,
    Win32::System::LibraryLoader::GetModuleHandleA, Win32::UI::WindowsAndMessaging::*,
};
use std::net::UdpSocket;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use std::future::Future;
use tauri::async_runtime::TokioJoinHandle;
pub type CallbackFn = Box<dyn Fn() + Send + 'static>;
// pub type CallbackFn = dyn FnOnce();
struct udp {
    socket: Option<UdpSocket>,
}
impl udp {
    fn new() -> Self {
        udp { socket: None }
    }
    fn bind(&mut self) {
        let socket = UdpSocket::bind("127.0.0.1:3315").unwrap();
        self.socket = Some(socket);
    }
    fn send(&self){
        if let Some(socket) = &self.socket {
            socket.send_to(b"{\"cmd\":\"__goals__\"}","127.0.0.1:12345").unwrap();
        }
    }
}
struct Channel{
    pub f: Option<CallbackFn>,
}

impl Channel{
    fn new(f:Option<CallbackFn>) -> Self{
        Channel{
            f: f
        }
    }

    fn set_fn(&mut self,f: Option<CallbackFn>){
        self.f = f;
    }

}

// static CHANNEL:       Mutex<Lazy<Channel>> = Mutex::new(Lazy::new(|| {Channel::new(None)}));
static SOCKET :       Lazy<Mutex<udp>> = Lazy::new(|| Mutex::new(udp::new()));

// pub fn set_fn(f: CallbackFn){
//     let mut tx = CHANNEL.lock().unwrap();
//     tx.set_fn(Some(f));
// }

pub fn hook() -> Result<()>{
    {
        let mut udp = SOCKET.lock().unwrap();
        udp.bind();
    }
    unsafe {
        let k_hook = SetWindowsHookExA(WH_KEYBOARD_LL, Some(k_callback1), HINSTANCE::default(), 0);
        let mut message = MSG::default();
        while GetMessageA(&mut message, HWND::default(), 0, 0).into() {
            DispatchMessageA(&message);
        }
        if let Ok(k_hook) = k_hook {
            if !k_hook.is_invalid(){
                UnhookWindowsHookEx(k_hook)?;
            }
            // println!("Hooked:{}",k_hook);
        }
        Ok(())
    }
}


extern "system" fn k_callback1(ncode: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    const CLIP_KEY: u16 = 13;
    unsafe {
        if wparam.0 as u32 == WM_KEYDOWN && ncode as u32 == HC_ACTION {
            let vk_code_inner = &*(lparam.0 as *const u16) as &u16;
        }
        if wparam.0 as u32 == WM_KEYUP && ncode as u32 == HC_ACTION {
            let up_vk_code_inner = &*(lparam.0 as *const u16) as &u16;
            if up_vk_code_inner == &CLIP_KEY {
                {
                    let udp = SOCKET.lock().unwrap();
                    udp.send();
                    dbg!(up_vk_code_inner);
                }
                // let tx = CHANNEL.lock().unwrap();
                // if let Some(f) = &tx.f {
                //     (f)();
                // }
            }
            // dbg!(up_vk_code_inner);
        }
        CallNextHookEx(HHOOK::default(), ncode, wparam, lparam)
    }
}
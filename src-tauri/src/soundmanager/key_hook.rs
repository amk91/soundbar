use std::sync::Mutex;

use once_cell::sync::Lazy;
use super::key_task::KeyTask;

pub static KEY_TASK: Lazy<Mutex<KeyTask>> = Lazy::new(
    || Mutex::new(KeyTask::default())
);
pub static APP_STATUS: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(true));

pub fn init_key_hook() {
    if cfg!(windows) {
        windows::init_key_hook();
    } else if cfg!(unix) {
        panic!("unix keyboard hook not implemented");
    }
}

#[cfg(target_os="windows")]
pub mod windows {
    use std::{
        ptr::null_mut,
        sync::{
            atomic::{
                AtomicPtr,
                Ordering,
            },
        }
    };

    use once_cell::sync::OnceCell;
    use winapi::{
        ctypes::c_int,
        shared::{
            minwindef::{HINSTANCE, WPARAM, LPARAM, LRESULT},
            windef::{HHOOK, HWND},
        },
        um::{
            winuser::{
                SetWindowsHookExW,
                GetMessageW,
                CallNextHookEx,
                UnhookWindowsHookEx,
                WH_KEYBOARD_LL,
                WM_KEYDOWN,
                WM_KEYUP,
                WM_SYSKEYDOWN,
                WM_SYSKEYUP,
                KBDLLHOOKSTRUCT,
                MSG,
            },
        }
    };

    use super::super::key_task::{KeyCode, SysKeyCode};

    pub static HOOK: OnceCell<AtomicPtr<HHOOK>> = OnceCell::new();
    
    pub unsafe extern "system" fn keyboard_hook(
        code: c_int,
        w_param: WPARAM,
        l_param: LPARAM
    ) -> LRESULT {
        if let Ok(false) = super::APP_STATUS.try_lock().as_deref() {
            UnhookWindowsHookEx(
                *HOOK.get_unchecked().load(Ordering::Relaxed)
            ) as LRESULT
        } else {
            let key_info: &KBDLLHOOKSTRUCT = unsafe {
                std::mem::transmute(l_param)
            };
    
            if let Ok(key_task) = super::KEY_TASK.lock().as_deref_mut() {
                match w_param as u32 {
                    WM_KEYDOWN => {
                        let code = KeyCode::try_from(key_info.vkCode);
                        key_task.key = if let Ok(key) = code {
                            Some(key)
                        } else {
                            let code = SysKeyCode::try_from(key_info.vkCode);
                            key_task.sys_key = if let Ok(key) = code {
                                Some(key)
                            } else {
                                None
                            };
                            None
                        };
                    }
                    WM_KEYUP => {
                        key_task.key = None;
                    },
                    //FIXME: ALT + key doesn't work if holding ALT
                    // down after the first stroke
                    WM_SYSKEYDOWN => {
                        let code = SysKeyCode::try_from(key_info.vkCode);
                        key_task.sys_key = match code {
                            Ok(key) => Some(key),
                            Err(_) => {
                                match key_task.sys_key {
                                    Some(SysKeyCode::LALT) | Some(SysKeyCode::RALT) => {
                                        let code = KeyCode::try_from(key_info.vkCode);
                                        key_task.key = if let Ok(key) = code {
                                            Some(key)
                                        } else {
                                            None
                                        };
                                        key_task.sys_key.clone()
                                    },
                                    _ => None,
                                }
                            }
                        };
                    }
                    WM_SYSKEYUP => key_task.sys_key = {
                        None
                    },
                    _ => {},
                }
            }
    
            CallNextHookEx(null_mut(), code, w_param, l_param)
        }
    }

    pub fn init_key_hook() {
        std::thread::spawn(|| {
            log::trace!("Key hook spawned");
            unsafe {
                let hook = &mut SetWindowsHookExW(
                    WH_KEYBOARD_LL,
                    Some(keyboard_hook),
                    0 as HINSTANCE,
                    0
                );

                HOOK.get_or_init(|| AtomicPtr::new(hook));
    
                log::trace!("Hook generated, {:?}, GetMessageW started", hook);
                let mut msg: MSG = std::mem::MaybeUninit::zeroed().assume_init();
                GetMessageW(&mut msg, 0 as HWND, 0, 0);
            };
        });
    }
}

#[cfg(target_os="unix")]
mod linux {
    //TODO:
}
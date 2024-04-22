#![windows_subsystem = "windows"]

use std::sync::mpsc;
use std::thread::{self};

use tray_item::{IconSource, TrayItem};
use windows_sys::Win32::Foundation::{HWND, POINT, WPARAM};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{MOD_SHIFT, VK_F11};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageW, GetWindowLongPtrW, SetWindowPos, MSG, SWP_NOZORDER, WM_HOTKEY,
    WS_VISIBLE,
};
use windows_sys::Win32::UI::{
    Input::KeyboardAndMouse::RegisterHotKey,
    WindowsAndMessaging::{
        GetForegroundWindow, IsZoomed, SetWindowLongPtrW, ShowWindow, TranslateMessage, GWL_STYLE,
        SW_MAXIMIZE, SW_RESTORE, WS_OVERLAPPEDWINDOW,
    },
};

const HOTKEY_ID: i32 = 1;

unsafe fn restore_window(hwnd: HWND) {
    let current_style = GetWindowLongPtrW(hwnd, GWL_STYLE) as u32;
    let new_style = current_style | WS_OVERLAPPEDWINDOW;
    SetWindowLongPtrW(hwnd, GWL_STYLE, new_style as isize);
    ShowWindow(hwnd, SW_RESTORE);
    SetWindowPos(hwnd, 0, 320, 180, 1280, 720, SWP_NOZORDER);
}

unsafe fn maximize_window(hwnd: HWND) {
    let current_style = GetWindowLongPtrW(hwnd, GWL_STYLE) as u32;
    let new_style = (current_style & !WS_OVERLAPPEDWINDOW) | WS_VISIBLE;
    SetWindowLongPtrW(hwnd, GWL_STYLE, new_style as isize);
    ShowWindow(hwnd, SW_MAXIMIZE);
}

fn toggle_window_style() {
    unsafe {
        let hwnd: HWND = GetForegroundWindow();
        let is_maximized = IsZoomed(hwnd) != 0;

        if is_maximized {
            restore_window(hwnd);
        } else {
            maximize_window(hwnd);
        }
    }
}

extern "system" fn register_keys() {
    unsafe {
        let hwnd: HWND = 0; // NULL, hotkey is global if hwnd is NULL
        if RegisterHotKey(hwnd, HOTKEY_ID, MOD_SHIFT, VK_F11 as u32) == 0 {
            eprintln!("Failed to register hotkey.");
            return;
        }

        println!("Hotkey Shift + F11 registered, press it to toggle window style.");

        let mut msg: MSG = MSG {
            hwnd: 0,
            message: 0,
            wParam: 0,
            lParam: 0,
            time: 0,
            pt: POINT { x: 0, y: 0 },
        };

        while GetMessageW(&mut msg, 0, 0, 0) != 0 {
            if msg.message == WM_HOTKEY && msg.wParam == HOTKEY_ID as WPARAM {
                toggle_window_style();
            }
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

enum Message {
    Quit,
}

fn main() {
    let mut tray = TrayItem::new("Oery Fullscreen", IconSource::Resource("id")).unwrap();
    tray.add_label("Oery Fullscreen").unwrap();

    let (tx, rx) = mpsc::sync_channel(1);

    let quit_tx = tx.clone();
    tray.add_menu_item("Quit", move || {
        quit_tx.send(Message::Quit).unwrap();
    })
    .unwrap();

    thread::spawn(move || {
        register_keys();
    });

    loop {
        match rx.recv() {
            Ok(Message::Quit) => {
                println!("Quit");
                break;
            }
            Err(_) => todo!(),
        }
    }
}

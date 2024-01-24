// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use screenshots::Screen;
use xcap::{Monitor, Window};

fn normalized(filename: &str) -> String {
    filename
        .replace("|", "")
        .replace("\\", "")
        .replace(":", "")
        .replace("/", "")
}

fn monitor_capture() -> Vec<Monitor> {
    let monitors = Monitor::all().unwrap();

    for monitor in &monitors {
        println!(
            "Monitor: {} {} {:?} {:?}",
            monitor.id(),
            monitor.name(),
            (monitor.x(), monitor.y(), monitor.width(), monitor.height()),
            (
                monitor.rotation(),
                monitor.scale_factor(),
                monitor.frequency(),
                monitor.is_primary()
            )
        );
        let image = monitor.capture_image().unwrap();

        image
            .save(format!("target/monitor-{}.png", normalized(monitor.name())))
            .unwrap();
    }

    monitors
}

fn window_capture() -> Vec<Window> {
    let windows = Window::all().unwrap();

    let mut i = 0;

    for window in &windows {
        // 最小化的窗口不能截屏
        if window.is_minimized() {
            continue;
        }

        println!(
            "Window: {:?} {:?} {:?}",
            window.title(),
            (window.x(), window.y(), window.width(), window.height()),
            (window.is_minimized(), window.is_maximized())
        );

        let image = window.capture_image().unwrap();
        image
            .save(format!(
                "target/window-{}-{}.png",
                i,
                normalized(window.title())
            ))
            .unwrap();

        i += 1;
    }

    windows
}

fn screen_capture() {
    let monitors = Screen::all().unwrap();

    for monitor in monitors {
        let image = monitor.capture().unwrap();
        println!("Display {:?}", monitor);
        image
            .save(format!("target/monitor-{}.png", monitor.display_info.id))
            .unwrap();
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn screenshots_test() -> String {
    screen_capture();

    "screenshots_test done!".into()
}

#[tauri::command]
fn xcap_test() -> String {
    monitor_capture();
    window_capture();

    screen_capture();

    "xcap_test done!".into()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![screenshots_test, xcap_test])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

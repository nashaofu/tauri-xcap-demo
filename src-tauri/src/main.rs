// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, path::Path};

use screenshots::Screen;
use tauri::{AppHandle, Runtime};
use xcap::{Monitor, Window};

fn normalized(filename: &str) -> String {
    filename
        .replace("|", "")
        .replace("\\", "")
        .replace(":", "")
        .replace("/", "")
}

fn monitor_capture(path: &Path) -> Vec<Monitor> {
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

        let p = path.join(format!("monitor-{}.png", normalized(monitor.name())));

        println!("path {:?}", p);

        image.save(p).unwrap();
    }

    monitors
}

fn window_capture(path: &Path) -> Vec<Window> {
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

        let p = path.join(format!("window-{}-{}.png", i, normalized(window.title())));
        println!("path {:?}", p);

        image.save(p).unwrap();

        i += 1;
    }

    windows
}

fn screen_capture(path: &Path) {
    let monitors = Screen::all().unwrap();

    for monitor in monitors {
        let image = monitor.capture().unwrap();
        println!("Display {:?}", monitor);
        image
            .save(path.join(format!("monitor-{}.png", monitor.display_info.id)))
            .unwrap();
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn screenshots_test<R: Runtime>(app: AppHandle<R>) -> String {
    screen_capture(&app.path_resolver().app_data_dir().unwrap());

    "screenshots_test done!".into()
}

#[tauri::command]
fn xcap_test<R: Runtime>(app: AppHandle<R>) -> String {
    monitor_capture(&app.path_resolver().app_data_dir().unwrap());
    window_capture(&app.path_resolver().app_data_dir().unwrap());

    "xcap_test done!".into()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![screenshots_test, xcap_test])
        .setup(|app| {
            let path = app.path_resolver().app_data_dir().unwrap();
            if !path.exists() {
                fs::create_dir_all(path).unwrap();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

mod commands;
mod utils;

use commands::{get_app_dir, greet};
use log::{debug, info};
use tauri::{
    webview::PageLoadPayload, App, Webview, WebviewUrl, WebviewWindowBuilder, Window, WindowEvent,
};
use tauri_plugin_log::{Target, TargetKind};
use utils::log_dir;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(logger().build())
        .invoke_handler(tauri::generate_handler![greet, get_app_dir])
        .setup(setup)
        .on_page_load(page_load_handler)
        .on_window_event(window_event_handler)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    info!("Setting up the app");

    let handle = app.handle();

    #[cfg(desktop)]
    {
        handle.plugin(tauri_plugin_window_state::Builder::default().build())?;
    }

    let mut builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default());

    #[cfg(desktop)]
    {
        builder = builder
            .user_agent(&format!("Hn app - {}", std::env::consts::OS))
            .title("Hacker News")
            .inner_size(1200., 800.)
            .min_inner_size(800., 600.)
            .resizable(true)
            .content_protected(true);
    }

    let webview = builder.build()?;

    #[cfg(debug_assertions)]
    webview.open_devtools();

    // initialize updater...

    Ok(())
}

// Fn(&Window<R>, &WindowEvent) + Send + Sync + 'static
fn window_event_handler(window: &Window, event: &WindowEvent) {
    debug!("Window event: {:?} on {:?}", event, window.label());

    if let WindowEvent::CloseRequested { api, .. } = event {
        if window.label() == "main" {
            info!("Close requested on {:?}", window.label());
            api.prevent_close();
            window.hide().unwrap();
        }
    }
}

// on_page_load is called when the webview is created.
// signature should be: Fn(&Webview<R>, &PageLoadPayload<'_>) + Send + Sync + 'static,
fn page_load_handler(webview: &Webview, _payload: &PageLoadPayload<'_>) {
    info!("Page loaded on {:?}", webview.label());
}

fn logger() -> tauri_plugin_log::Builder {
    tauri_plugin_log::Builder::default()
        .targets([
            Target::new(TargetKind::Webview),
            Target::new(TargetKind::Folder {
                path: log_dir(),
                file_name: None,
            }),
            Target::new(TargetKind::Stdout),
        ])
        .level(log::LevelFilter::Info)
}

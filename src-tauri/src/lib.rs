use std::path::PathBuf;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn format_file(file: PathBuf) -> Result<String, String> {
    fn format_markdown(markdown: &str) -> String {
        let parser = pulldown_cmark::Parser::new_ext(markdown, pulldown_cmark::Options::all());
        let mut html = String::new();
        pulldown_cmark::html::push_html(&mut html, parser);
        html
    }

    match std::fs::read_to_string(&file) {
        Ok(contents) => {
            match file.extension().and_then(|ext| ext.to_str()) {
                Some("md") => Ok(format_markdown(&contents)),
                _ => {
                    let encoded = html_escape::encode_text(contents.as_str());
                    Ok(format!("<pre>{}</pre>", encoded))
                },
            }
        }
        Err(error) => Err(error.to_string()),
    }
}
#[tauri::command]
fn path_argument() -> Option<PathBuf> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let path_arg = PathBuf::from(&args[1]);
        Some(std::path::absolute(path_arg.clone()).unwrap_or(path_arg))
    } else {
        None
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    tauri::Builder::default()
        .setup(|app| {
            let title = format!("Markdown Viewer v{}", env!("CARGO_PKG_VERSION"));

            app.get_webview_window("main")
                .expect("main window not found")
                .set_title(&title)?;
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![format_file, path_argument])
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}

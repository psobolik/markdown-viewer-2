use std::path::PathBuf;

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
            if file.extension().unwrap().eq_ignore_ascii_case("md") {
                Ok(format_markdown(&contents))
            } else {
                let encoded = html_escape::encode_text(contents.as_str());
                Ok(format!("<pre>{}</pre>", encoded.to_string()))
            }
        }
        Err(error) => Err(error.to_string()),
    }
}
#[tauri::command]
fn path_argument() -> Option<PathBuf> {
    const FILE_ID: &str = "MARKDOWN_FILE";

    let matches = clap::Command::new("Markdown Viewer")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A simple Markdown file viewer")
        .arg(
            clap::Arg::new(FILE_ID)
                .help("Markdown file to view")
                .value_parser(clap::value_parser!(PathBuf)),
        )
        .get_matches();
    match matches.get_one::<PathBuf>(FILE_ID) {
        Some(file) => {
            if let Ok(file) = std::env::current_dir().unwrap().join(file).canonicalize() {
                Some(PathBuf::from(file))
            } else {
                Some(PathBuf::from(file))
            }
        }
        None => None,
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![format_file, path_argument])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

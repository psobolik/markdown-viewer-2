use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
// use leptos::logging::log;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], catch, js_name=invoke)]
    async fn invoke_result(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name=invoke)]
    async fn invoke_option(cmd: &str) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct FormatFileArgs {
    file: PathBuf,
}

#[component]
pub fn App() -> impl IntoView {
    let (path, set_path): (ReadSignal<Option<PathBuf>>, WriteSignal<Option<PathBuf>>) =
        signal(None);

    fn path_for_display(path: Option<PathBuf>) -> String {
        leptos::logging::log!("path for display: {path:?}");
        if let Some(path) = path {
            path.display().to_string()
        } else {
            String::from("No file specified")
        }
    }
    let header = Memo::new(move |_| path_for_display(path.get()));

    async fn path_argument() -> Option<PathBuf> {
        let path_argument = invoke_option("path_argument").await;
        match path_argument.as_string() {
            Some(path) => Some(PathBuf::from(path)),
            None => None,
        }
    }
    // async fn path_argument() -> Option<PathBuf> {
    //     let path_argument = invoke_option("path_argument").await;
    //     match path_argument.as_string() {
    //         Some(path) => Some(PathBuf::from(path)),
    //         None => None,
    //     }
    // }

    async fn format_file(path: Option<PathBuf>) -> Result<String, String> {
        if let Some(path) = path {
            let args = serde_wasm_bindgen::to_value(&FormatFileArgs { file: path }).unwrap();
            let text = invoke_result("format_file", args).await;
            match text {
                Ok(text) => Ok(text.as_string().unwrap()),
                Err(error) => Err(error.as_string().unwrap()),
            }
        } else {
            Ok(String::default())
        }
    }

    let file_resource = LocalResource::new(move || path_argument());
    let content_resource = LocalResource::new(move || format_file(path.get()));

    view! {
        <main class="container">
            <div id="title">{header}</div>
            {move || Suspend::new(async move {
                let markdown_file = file_resource.await;
                set_path.set(markdown_file);
            })}
            {move || Suspend::new(async move {
                let content = content_resource.await;
                view! {
                    <div id="viewer" inner_html=content.clone().ok()></div>
                    <div id="error-message">{content.err()}</div>
                }
            })}
        </main>
    }
}

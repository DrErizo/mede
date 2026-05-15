mod downloader;
use std::process::Command;
use downloader::Downloader;
use dioxus::prelude::*;
use serde_json;
use dioxus_desktop::{Config, WindowBuilder, LogicalSize};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    AddressBar {},
    #[route("/editor")]
    Editor {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn getVideoInfo(info: &str ,url: &str) -> Option<String> {
    let output = Command::new("yt-dlp")
        .args([
            "--print-json",
            "--skip-download",
            "--no-playlist",
            url,
        ])
        .output()
        .ok()?;
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).ok()?;
    json[info].as_str().map(|s| s.to_string())
}
fn main() {
    dioxus::LaunchBuilder::new()
        .with_cfg(desktop! {
            Config::new().with_window(
                WindowBuilder::new()
                    .with_title("Mede Downloader")
                    .with_inner_size(LogicalSize::new(800.0, 600.0))
            )
        })
        .launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}
#[component]
pub fn AddressBar() -> Element {
    let mut url = use_signal(|| String::new());
    let mut status = use_signal(|| "Idle".to_string());
    let mut only_sound = use_signal(|| false);
    let mut editor_mode = use_signal(|| false);
    let mut attach_metadata = use_signal(|| false);

    rsx! {
        div {
            id: "userInterface",

            input {
                id: "addressBar",
                r#type: "text",
                placeholder: "link here....",
                value: "{url}",
                oninput: move |e| url.set(e.value()),
            }
            div {
                id: "togglers",

                div {
                    h1 { "Editor Mode:" }
                    label {
                        class: "switch",
                        input { 
                            r#type: "checkbox",
                            checked: editor_mode(),
                            onchange: move |e| editor_mode.set(e.checked()),
                        }
                        span { class: "slider round" }
                    }
                }

                div {
                    h1 { "Sound Only:" }
                    label {
                        class: "switch",
                        input {
                            r#type: "checkbox",
                            checked: only_sound(),
                            onchange: move |e| only_sound.set(e.checked()),
                        }
                        span { class: "slider round" }
                    }
                }

                div {
                    h1 { "Attach Metadata:" }
                    label {
                        class: "switch",
                        input {
                            r#type: "checkbox",
                            checked: attach_metadata(),
                            onchange: move |e| attach_metadata.set(e.checked()),
                        }
                        span { class: "slider round" }
                    }
                }
            }
            button {
                id: "downloadBtn",
                onclick: move |_| {
                    let url_value = url();
                    let only_sound_val = only_sound();

                    if url_value.trim().is_empty() {
                        status.set("Please enter a URL".to_string());
                        return;
                    }

                    let Some(path) = rfd::FileDialog::new()
                        .set_title("Save as")
                        .set_file_name(getVideoInfo("filename",&url_value).unwrap().replace(" ", "-"))
                        .save_file()
                    else {
                        status.set("No file selected".to_string());
                        return;
                    };

                    let dir = path.parent().unwrap().to_path_buf();
                    let output_file = path.file_name().unwrap().to_string_lossy().to_string();

                    let downloader = Downloader::new(dir);
                    let mut status = status.clone();

                    spawn(async move {
                        status.set("Downloading...".to_string());

                        let result = tokio::task::spawn_blocking(move || {
                            downloader.download(url_value, output_file,only_sound_val)
                        })
                        .await;

                        match result {
                            Ok(Ok(path)) => status.set(format!("Downloaded to {}", path.display())),
                            Ok(Err(e))   => status.set(format!("Download failed: {}", e)),
                            Err(e)       => status.set(format!("Worker thread panicked: {}", e)),
                        }
                    });
                    
                    if editor_mode() {
                        navigator().push(Route::Editor{});
                    }
                },
                "Download"
            }
 
 

            div { p { "Status: {status}" } }
        }
    }
}
#[component]
pub fn Editor() -> Element {
    rsx! { div { "editor mode coming soon" } }
}

use dioxus::prelude::*;
use crate::AppState;

#[component]
pub fn Editor() -> Element {
    let state = use_context::<Signal<AppState>>();

    rsx! { 
        div {
            id:"titlebar",
            div {
                id:"info",
                p { "{state.read().video_title.read()}" }
                p { "{state.read().video_path.read()}" }
            }
            button { id:"export", "Export" }
        }
        div { id:"preview",
            div {
                canvas {
                
                }
            }
        } 
        div { id:"controls",
            button { id: "skipBack", "↩" },
            button { id: "play", "▶︎" },
            button { id: "skipFront", "↪" }
        } 
    }
}

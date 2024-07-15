use dioxus::prelude::*;
use tracing::info;

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    let (count, set_count) = use_state(&cx, || 0);

    cx.render(rsx! {
        h1 { "High-Five counter: {count}" }
        button { onclick: move |e|{
            info!("incrementing count: {e:?}");
            set_count(count + 1);
        },
            "Up high!" }
        button { onclick: move |_| set_count(count - 1), "Down low!" }
    })
}
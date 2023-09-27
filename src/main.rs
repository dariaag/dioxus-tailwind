#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

fn main() {
    // launch the web app
    //dioxus_web::launch(App);

    #[cfg(any(windows, unix))]
    dioxus_desktop::launch_cfg(
        App,
        dioxus_desktop::Config::new().with_custom_head(
            r#"<link rel="stylesheet" href="./dist/public/tailwind.css">"#.to_string(),
        ),
    );
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        link { rel: "stylesheet", href: "./public/tailwind.css" },
        div {
            class: "grid h-screen px-4 bg-white place-content-center",
            div{
                class:"text-center",
                p{
                    class:"text-2xl font-bold tracking-tight text-gray-900 sm:text-4xl",
                    "Dioxus 💙 Tailwind"
                },
                p{
                    class: "mt-4 text-gray-500",
                    "test"
                }
                a{
                    class:"inline-block px-5 py-3 mt-6 text-sm font-medium text-white bg-indigo-600 rounded hover:bg-indigo-700 focus:outline-none focus:ring",
                    "Go Back Home"
                }
            }

        }
    })
}

use dioxus::prelude::*;
mod views;
use views::{Home,Meme,Content,Resume};
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    LandingPage {},
    #[route("/home")]
    Home{},
    #[route("/cat")]
    Meme{},
    #[route("/portfolio")]
    Content{},
    #[route("/resume")]
    Resume{},
}

const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const YINYANG: Asset = asset!("/assets/misc/yinyang.svg");
const BG: Asset = asset!("/assets/misc/background_scene.jpg");
const MEGRIM : Asset = asset!("assets/fonts/Megrim-Regular.ttf");


fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Style{
            "@font-face {{ font-family: 'Megrim'; src: url({MEGRIM}) format('truetype'); }}"
        }
        Router::<Route> {}
    }
}

#[component]
fn LandingPage() -> Element {
    rsx! {
        main{
            class: "grid grid-cols-3 grid-rows-3 h-screen",
            div {
                class: "fixed inset-0 -z-10 bg-cover \
                bg-center bg-no-repeat",
                style: "background-image: url({BG});",
                aria_hidden: "true",
            }
            div{
                class: " row-start-3 col-start-2 place-self-center \
                glass-card align-middle font-[Megrim] text-white hover:text-black \
                p-2 text-6xl bg-black/20 hover:bg-white/20 peer/peace",
                Link{
                    to: Route::Home{},"Enter the peace"
                }
            }
            img { src: YINYANG, alt: "Logo", class: "size-32 row-start-1 col-start-2 row-span-2 \
            place-self-center  rounded-full p-0 \
            transition-all ease-in-out peer-hover/peace:rotate-180 \
            peer-hover/peace:shadow-[0_0_50px_10px_white]"
            }
        }
    }
}


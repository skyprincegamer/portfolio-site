use dioxus::prelude::*;
use crate::Route;

const MEGRIM : Asset = asset!("assets/fonts/Megrim-Regular.ttf");
const KIWI : Asset = asset!("assets/fonts/KiwiMaru-Regular.ttf");
const AKASH : Asset = asset!("assets/misc/self_irl.jpeg");

const SPACE : Asset = asset!("assets/fonts/SpaceMono-Regular.ttf");
const POINTING : Asset = asset!("assets/misc/pointing.png");

#[component]
pub fn Home() -> Element {
    let font_face_things = format!(
        "@font-face {{ font-family: 'Megrim'; src: url({MEGRIM}) format('truetype'); }}
        @font-face {{ font-family: 'Kiwi'; src: url({KIWI}) format('truetype'); }}
        @font-face {{ font-family: 'MonoS'; src: url({SPACE}) format('truetype'); }}
"
    );
    let is_active = use_resource(|| async move {
        let mut eval =
            document::eval("dioxus.send(localStorage.getItem('is_active') === 'true');");
        eval.recv::<bool>().await.unwrap_or(false)
    });
    rsx!{
        document::Style {
            {font_face_things}
        }
        main{
            class: "font-[MonoS]",
            section{
            class: "bg-black min-h-screen grid grid-rows-[1fr_2fr_2fr] grid-cols-3",
                h1{
                    class: "font-[Kiwi] row-start-2 col-start-2 \
                    place-self-center text-white text-9xl text-center",
                    "泰"
                }
                footer{
                    class: "font-[Megrim] row-start-3 col-start-2 place-self-center text-white text-4xl",
                    "Scroll down to explore"
                }
            }
            section{
                class: "gradient-pulse h-screen grid grid-rows-3 grid-cols-3",
                h1{
                    class:"row-start-2 col-start-1 col-span-2 place-self-center text-white text-4xl p-12",
                    "I am a very humble person. I do not want to brag about myself. \
                    So let's talk about something else."
                }
                img{
                    alt : "Photograph of Akash Kumar",src:AKASH,class: "row-start-2 col-start-3 size-64 \
                    place-self-center rounded-full object-cover object-top"
                }
            }
            section{
                class: "bg-[#1E293B] min-h-screen grid grid-rows-3 grid-cols-3",
                h1{
                    class:"bg-[#020617] row-start-1 col-start-2 place-self-center \
                    text-white text-2xl rounded-md p-2 whitespace-nowrap",
                    "By the way, who are ",
                    span{
                        class: "relative",
                        Link{
                        class: "duration-500 leading-normal underline",
                        to: Route::Meme{},
                        "you?"
                        }
                        img {
                            class: "absolute max-w-none sm:h-64 sm:left-8 sm:-top-8 h-40 left-10 -top-4",
                            src: POINTING
                        }
                    }
                }
                if is_active() == Some(true){
                    footer{
                        class: "font-[Megrim] ftransition-all duration-500 row-start-3 col-start-2 \
                        place-self-center align-middle text-white rounded-2xl shadow-lg/50 \
                       hover:shadow-xl/100 text-center p-2 text-3xl bg-black/20",
                        Link{
                            to: Route::Content{},"Ok. You have unlocked the real portfolio\u{1FAE1}."
                        }
                    }
                }
            }
        }
    }
}

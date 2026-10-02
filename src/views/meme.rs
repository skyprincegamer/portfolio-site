use dioxus::prelude::*;

const CAT : Asset = asset!("assets/misc/cat.mp4");
#[component]
pub fn Meme() -> Element {
    use_effect(move || {
        document::eval("localStorage.setItem('is_active', 'true');");
    });
    rsx!{
        video {
            src: CAT,
            autoplay: true,
            controls: true,
            r#loop: true,
            playsinline: true,
            muted:true,
            class: "h-screen w-full bg-black"
        }
    }
}
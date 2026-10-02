use dioxus::prelude::*;

const RESUME_PDF: Asset = asset!("assets/my_resume-1.pdf");

#[component]
pub fn Resume() -> Element {
    rsx! {
        div {
            class: "w-screen h-screen bg-[#020617] flex flex-col items-center justify-center text-white gap-6",

            h1 {
                class: "text-3xl font-mono",
                "Grab a copy of my resume."
            }

            a {
                href: "{RESUME_PDF}",
                download: "Akash_Kumar_Resume.pdf",
                class: "px-6 py-3 bg-red-600 hover:bg-red-500 rounded-full font-bold text-lg transition-all hover:scale-105",
                "Download PDF"
            }
        }
    }
}
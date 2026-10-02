use dioxus::prelude::*;
use crate::Route;
const KIWI : Asset = asset!("assets/fonts/KiwiMaru-Regular.ttf");

const SPACE : Asset = asset!("assets/fonts/SpaceMono-Regular.ttf");
const SKILL_TREE : Asset = asset!("assets/content/skills.svg");
const MEGRIM : Asset = asset!("assets/fonts/Megrim-Regular.ttf");
const KDE : Asset = asset!("assets/content/kde.png");

const SYNCO : Asset = asset!("assets/fonts/Syncopate-Regular.ttf");
const LATO : Asset = asset!("assets/fonts/Lato-Regular.ttf");

const BBH : Asset = asset!("assets/fonts/BBHBartle-Regular.ttf");

#[component]
pub fn Content() -> Element {
    let font_face_things = format!(
        "@font-face {{ font-family: 'Kiwi'; src: url({KIWI}) format('truetype'); }}
        @font-face {{ font-family: 'MonoS'; src: url({SPACE}) format('truetype'); }}
        @font-face {{ font-family: 'Megrim'; src: url({MEGRIM}) format('truetype'); }}
        @font-face {{ font-family: 'Lato'; src: url({LATO}) format('truetype'); }}
        @font-face {{ font-family: 'Synco'; src: url({SYNCO}) format('truetype'); }}
        @font-face {{ font-family: 'BBH'; src: url({BBH}) format('truetype'); }}
        "
    );
    rsx! {
        document::Style{
            {font_face_things}
        }
        div {
            class: "min-h-screen bg-[#020617] text-white p-2 m-2 md:p-16 font-sans overflow-y-auto",

            // Hero / Header Section
            header {
                class: "text-center",
                hgroup{
                    h1 {
                        class: "text-4xl md:text-5xl font-bold text-pulse font-[BBH] mt-4 mb-2",
                        "Akash Kumar"
                    }
                    p {
                        class: "text-xl text-gray-400 font-[Lato] mb-8",
                        "Full-Stack & Systems Developer specializing in Rust, Android Native, and high-performance applications."
                    }
                    blockquote{
                        class: "font-['MonoS'] text-center text-3xl text-white my-2",
                        "\"My true strength lies not in my knowledge, but rather in my ability to \
                        learn and adapt in difficult situations.\""
                    }
                    blockquote{
                        class: "font-[Kiwi] text-center text-4xl text-white gradient-water-text mt-2 mb-6",
                        "「私の知識は真の実力ではありません。むしろ、適応力こそが私の真の実力です。」"
                    }
                    p{
                        class: "font-[Lato] text-center text-xl text-white",
                        "- Akash Kumar"
                    }

                }
                nav {
                    class: "grid grid-cols-4 my-4 gap-4 w-screen relative left-1/2 -translate-x-1/2 p-4 bg-[#0F172A]",
                    a {
                        href: "mailto:akyt3108@gmail.com",
                        class:"link_button",
                        "akyt3108@gmail.com"
                    }
                    a {
                        href: "https://github.com/skyprincegamer",
                        target: "_blank",
                        class:"link_button",
                        "GitHub"
                    }
                    a {
                        href: "https://linkedin.com/in/akash-kumar7777",
                        target: "_blank",
                        class:"link_button",
                        "LinkedIn"
                    }
                    Link {
                        to: Route::Resume{},
                        class: "link_button",
                        "View Resume"
                    }
                }
            }
            main {
                section{
                    class:"grid",
                    h1{
                        class:"text-3xl font-semibold mb-8 text-gray-100 font-[Synco]",
                        "Skill Tree"
                    }
                    img{
                            src: SKILL_TREE,
                            class: "h-screen place-self-center tree-pulse"
                    }
                }
                hr{
                    class:"m-4"
                }
                // Projects Section
                section {
                    class: "mb-8",
                    h2 { class: "text-3xl font-semibold mb-8 text-gray-100 font-[Synco]", "Projects" }
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",

                        // FinSigma
                        div { class: "bg-[#0F172A] p-2 m-2 rounded-2xl border border-gray-800 hover:border-gray-600 transition-all flex flex-col h-full shadow-lg",
                            div { class: "flex justify-between items-start mb-2",
                                h3 { class: "text-2xl font-bold font-[Megrim]", "FinSigma" }
                                a { href: "https://finsigma.vercel.app", target: "_blank", class: "text-sm text-gray-400 font-[Synco] hover:text-white transition-colors", "Live ↗" }
                            }
                            p { class: "text-sm  mb-6 font-[MonoS]", "Next.js, TypeScript, PostgreSQL, Prisma, Tailwind" }
                            ul { class: "list-disc list-outside ml-4 text-gray-400 font-[Lato] text-sm space-y-3 flex-grow leading-relaxed",
                                li { "Engineered an ACID-compliant simulated trading platform with live market pricing to strictly enforce balances and prevent ledger desynchronization during high-concurrency trades." }
                                li { "Optimized financial instrument discovery by implementing a millisecond-latency fuzzy-search engine using PostgreSQL trigram similarity operators (<% and <<->)." }
                            }
                        }

                        // CloudClock
                        div { class: "bg-[#0F172A] p-2 m-2 rounded-2xl border border-gray-800 hover:border-gray-600 transition-all flex flex-col h-full shadow-lg",
                            div { class: "flex justify-between items-start mb-2",
                                h3 { class: "text-2xl font-bold font-[Megrim]", "CloudClock" }
                                a { href: "https://github.com/skyprincegamer/cloud_clock", target: "_blank", class: "text-sm text-gray-400 font-[Synco] hover:text-white transition-colors", "Repo ↗" }
                            }
                            p { class: "text-sm mb-6 font-[MonoS]", "Kotlin, Jetpack Compose, Supabase Realtime, Room DB" }
                            ul { class: "list-disc list-outside ml-4 text-gray-400 font-[Lato] text-sm space-y-3 flex-grow leading-relaxed",
                                li { "Eliminated alarm polling and achieved instantaneous bidirectional sync across 3+ concurrent physical and emulated devices using a cloud-sync engine triggered by Supabase Realtime." }
                                li { "Guaranteed Doze-mode-resilient alarm delivery using localized UTC conversions and precise RTC_WAKEUP intents." }
                            }
                        }

                        // Oxy Bars
                        div { class: "bg-[#0F172A] p-2 m-2 rounded-2xl border border-gray-800 hover:border-gray-600 transition-all flex flex-col h-full shadow-lg md:col-span-2",
                            div { class: "flex justify-between items-start mb-2",
                                h3 { class: "text-2xl font-bold font-[Megrim]", "Oxy Bars" }
                                a { href: "https://github.com/skyprincegamer/oxy_bars", target: "_blank", class: "text-sm text-gray-400 font-[Synco] hover:text-white transition-colors", "Repo ↗" }
                            }
                            p { class: "text-sm mb-6 font-[MonoS]", "Rust, cpal, rustfft, Macroquad, TOML" }
                            ul { class: "list-disc list-outside ml-4 text-gray-400 font-[Lato] text-sm space-y-3 flex-grow leading-relaxed",
                                li { "Solved spectral leakage and ambient noise pollution in real-time audio analysis by implementing Blackman-Nuttall windowing and live spectrum subtraction." }
                                li { "Processed 44.1kHz audio streams with sub-16ms visual render cycles, achieving 22us lock-free FFT pipeline cadences via atomic memory ordering." }
                            }
                        }
                    }
                }

                // Open Source Section
                section {
                    class: "mb-8",
                    h2 { class: "text-3xl font-semibold mb-8 text-gray-100 font-[Synco]", "Open Source Contributions" }
                    article { class: "bg-[#0F172A] rounded-2xl border border-gray-800 shadow-lg grid grid-rows-auto grid-cols-auto",

                        hgroup{
                            class:"col-start-1 h-fit pl-2",
                            h3 { class: "text-2xl font-bold mb-1 font-[Megrim]", "KDE KleverNotes" }
                            p { class: "text-sm mb-6 font-[MonoS]", "QML / C++ Note-Taking App (KDE Invent)" }
                        }
                        ul {
                            class: "row-start-2 list-disc list-outside text-gray-400 font-[Lato] text-sm space-y-3 pl-4 pb-2 leading-relaxed",
                            li { "Engineered a dynamic note-sorting engine with preemptive state-saving to prevent data loss during live UI configuration changes." }
                            li { "Implemented a draggable canvas pan tool with coordinate bound limits to prevent out-of-bounds rendering on the painting page." }
                            li { "Expanded painting capabilities by developing robust shape-drawing features and real-time elliptical previews." }
                            li { "Resolved critical file path resolution bugs for saved painted images, contributing 240+ lines of robust QML and C++ across all 4 MRs." }
                        }
                        img{
                            src:KDE,
                            class:"h-32 col-start-2 pb-2 row-span-2 place-self-center"
                        }
                    }
                }
                // Education Section
                section {
                    h2 { class: "text-3xl font-semibold mb-8 text-gray-100 font-[Synco]", "Education" }
                    div { class: "flex flex-col md:flex-row justify-between items-start md:items-center bg-gradient-to-br from-[#0F172A] to-[#1E293B] p-2 m-2 rounded-2xl border border-gray-800 shadow-lg",
                        div {
                            h3 { class: "text-xl font-bold mb-1 font-[Megrim]", "IIIT Gwalior" }
                            p { class: "text-gray-400 font-[Lato]", "Integrated B.Tech + M.Tech in Information Technology" }
                        }
                        div { class: "mt-6 md:mt-0 md:text-right flex flex-col gap-1",
                            p { class: "text-gray-400 font-[Lato] text-sm bg-black/20 py-1 px-3 rounded-full self-start md:self-end border border-gray-700/50", "August,2024 - May,2029" }
                            p { class: "text-emerald-400 font-bold mt-2 font-[MonoS]", "CGPA: 8.18" }
                        }
                    }
                }
                hr{
                    class:"m-4"
                }
            }
        }
    }
}
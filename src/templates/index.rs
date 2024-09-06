use crate::capsules::{Container, SPONSOR};
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

fn index_page<G: Html>(cx: Scope, state: IndexState) -> View<G> {
    view! {
        cx,
        Container(title = "AAEO") {
            // Grid rows are sized on mobile to contain the images
            section(id = "welcome", class = "w-full min-h-screen grid grid-rows-[1fr_1fr_20rem] md:grid-rows-1 md:grid-cols-3") {
                div(class = "flex flex-col justify-center row-span-2 md:col-span-2 text-center md:text-left md:pl-8 lg:pl-16 xl:pl-20 md:px-4 bg-welcome bg-cover pt-8 md:pt-0") {
                    h1(class = "text-3xl md:text-5xl text-neutral-600 font-extrabold mb-2 md:mb-6 mt-16 md:mt-10 max-w-4xl") { "The Australasian Economics Olympiad" }
                    p(class = "text-neutral-600 text-xl md:text-3xl font-extrabold") { "Discover the Economist in You" }

                    div(class = "font-semibold flex justify-center md:justify-start mt-2") {
                        a(
                            class = "rounded-lg bg-amber-500 hover:bg-amber-400 transition-colors duration-200 text-lg p-2 px-4 text-white mr-2",
                            href = "#contact"
                        ) { "Contact us" }
                        a(
                            class = "rounded-lg border-2 border-neutral-600 hover:bg-amber-400 hover:border-amber-400 transition-colors duration-200 text-lg p-2 px-4 text-neutral-600 hover:text-white",
                            href = "#competition"
                        ) { "Learn more" }
                    }

                    div(class = "w-full flex flex-col justify-center md:justify-start") {
                        div(class = "text-base md:text-lg rounded-xl p-3 md:p-6 bg-slate-100/30 backdrop-blur-lg my-4 mx-4 md:ml-0 md:mr-4 md:max-w-xl") {
                            p {
                                "Congratulations to all those who participated in the 2024 AAEO! You can see details and photos from the event "
                                a(
                                    class = "underline",
                                    href = "competition/2024",
                                ) { "here" }
                                ", and we look forward to a fantastic competition in 2025!"
                            }
                        }
                    }
                }
                div(class = "h-full w-full") {
                    span(class = "flex justify-center items-center h-full w-full text-white bg-cover bg-primary-1") {}
                }
            }
            section(id = "competition", class = "w-full min-h-screen grid grid-rows-[1fr_1fr_20rem] md:grid-rows-1 md:grid-cols-3") {
                div(class = "h-full w-full") {
                    span(class = "flex justify-center items-center h-full w-full text-white bg-cover bg-primary-2") {}
                }
                div(class = "row-start-1 row-span-2 md:col-span-2 md:col-start-2 flex flex-col justify-center items-center p-4 py-8 bg-competition bg-cover") {
                    div(class = "prose prose-slate prose-h1:underline md:text-right md:ml-20 my-6") {
                        div(dangerously_set_inner_html = &format!("<h1>Competition</h1>{}", state.competition_html)) {}
                        // NOTE: Schedule button is hidden until competition starts
                        // a(
                        //     class = "rounded-lg border-2 border-neutral-600 hover:bg-amber-400 hover:border-amber-400 transition-colors duration-200 text-lg p-2 px-4 text-neutral-600 hover:text-white",
                        //     href = "schedule"
                        // ) { "Schedule" }
                    }
                }
            }
            div(class = "w-full min-h-screen grid grid-rows-[1fr_1fr] md:grid-rows-1 md:grid-cols-2") {
                div(class = "bg-about bg-cover px-4") {
                    section(id = "contact", class = "w-full flex flex-col items-center pt-8") {
                        div(class = "max-w-4xl text-left") {
                            h1(class = "text-4xl") { "Get Involved" }
                            div(class = "prose prose-slate prose-a:font-normal") {
                                div(dangerously_set_inner_html = &state.contact_html) {}
                            }
                        }
                    }
                    section(id = "about", class = "w-full flex flex-col items-center pt-4") {
                        div(class = "max-w-4xl text-left") {
                            h1(class = "text-4xl") { "About Us" }
                            p(class = "mt-3 mb-6 prose text-left") { (state.about) }
                        }
                    }
                }
                div(class = "h-full w-full") {
                    span(class = "flex justify-center items-center h-full w-full text-white bg-cover bg-primary-3") {}
                }
            }
            section(id = "sponsors", class = "flex flex-col justify-center items-center py-8") {
                h1(class = "text-4xl md:mb-1") { "Our Sponsors" }
                (SPONSOR.widget(cx, "unsw", ()))
                (SPONSOR.widget(cx, "ciee", ()))
                (SPONSOR.widget(cx, "elite", ()))
            }
        }
    }
}

#[derive(Serialize, Deserialize, UnreactiveState, Clone)]
struct IndexState {
    /// The HTML to be interpolated into the "competition" section of the site.
    competition_html: String,
    /// The text to go at the header of the about section.
    about: String,
    /// The HTML for the contact section.
    contact_html: String,
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Australasian Economics Olympiad" }
    }
}

#[engine_only_fn]
async fn get_build_state(
    _: StateGeneratorInfo<()>,
) -> Result<IndexState, BlamedError<std::io::Error>> {
    use pulldown_cmark::{html, Options, Parser};

    let mut options = Options::empty();
    options.insert(Options::ENABLE_SMART_PUNCTUATION);

    let competition_md = std::fs::read_to_string("content/competition.md")?;
    let mut competition_html = String::new();
    let md_parser = Parser::new_ext(&competition_md, options);
    html::push_html(&mut competition_html, md_parser);

    let contact_md = std::fs::read_to_string("content/contact.md")?;
    let mut contact_html = String::new();
    let md_parser = Parser::new_ext(&contact_md, options);
    html::push_html(&mut contact_html, md_parser);

    let about = std::fs::read_to_string("content/about.txt")?;

    Ok(IndexState {
        competition_html,
        contact_html,
        about,
    })
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index")
        .view_with_unreactive_state(index_page)
        .head(head)
        .build_state_fn(get_build_state)
        .build()
}

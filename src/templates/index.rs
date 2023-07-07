use crate::capsules::{Container, SPONSOR};
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

fn index_page<G: Html>(cx: Scope, state: IndexState) -> View<G> {
    view! {
        cx,
        Container(title = "AAEO") {
            // Grid rows are sized on mobile to contain the images
            section(id = "welcome", class = "w-full h-screen grid grid-rows-[1fr_1fr_20rem] md:grid-rows-1 md:grid-cols-3") {
                div(class = "flex flex-col justify-center row-span-2 md:col-span-2 text-center md:text-left md:pl-8 lg:pl-16 xl:pl-20 md:px-4 bg-welcome bg-cover") {
                    h1(class = "text-3xl md:text-5xl text-neutral-600 font-extrabold mb-2 md:mb-6 mt-16 md:mt-10 max-w-4xl") { "The Australasian Economics Olympiad" }
                    h3(class = "text-neutral-600 text-xl md:text-3xl font-extrabold") { "Discover the Economist in You" }

                    div(class = "font-semibold flex justify-center md:justify-start mt-2") {
                        a(
                            class = "rounded-lg bg-amber-500 hover:bg-amber-400 transition-colors duration-200 text-lg p-2 px-4 text-white mr-2",
                            href = "mailto:admin@aeo.org.au?subject=An Inquiry about the AAEO" // TODO Change this when the AAEO has an email address
                        ) { "Contact us" }
                        a(
                            class = "rounded-lg border-2 border-neutral-600 hover:bg-amber-400 hover:border-amber-400 transition-colors duration-200 text-lg p-2 px-4 text-neutral-600 hover:text-white",
                            href = "#competition"
                        ) { "Learn more" }
                    }
                }
                div(class = "h-full w-full") {
                    span(class = "flex justify-center items-center h-full w-full bg-blue-700 text-white") {
                        img(class = "object-cover h-full w-full", src = ".perseus/static/people.jpg") {}
                    }
                }
            }
            section(id = "competition", class = "w-full min-h-screen grid grid-rows-[1fr_1fr_20rem] md:grid-rows-1 md:grid-cols-3") {
                div(class = "h-full w-full") {
                     span(class = "flex justify-center items-center h-full w-full bg-blue-700 text-white") {
                         img(class = "object-cover h-full w-full", src = ".perseus/static/compass.jpg") {}
                    }
                }
                div(class = "row-start-1 row-span-2 md:col-span-2 md:col-start-2 flex flex-col justify-center items-center p-4 py-8 bg-competition bg-cover") {
                    div(class = "prose prose-slate prose-h1:underline md:text-right md:ml-20 my-6") {
                        div(dangerously_set_inner_html = &format!("<h1>Competition</h1>{}", state.competition_html)) {}
                        a(
                            class = "rounded-lg border-2 border-neutral-600 hover:bg-amber-400 hover:border-amber-400 transition-colors duration-200 text-lg p-2 px-4 text-neutral-600 hover:text-white",
                            href = "schedule"
                        ) { "Schedule" }
                    }
                }
            }
            section(id = "about", class = "w-full min-h-screen py-8 flex flex-col justify-center items-center w-full") {
                div(class = "max-w-4xl text-center") {
                    h1(class = "text-4xl") { "About Us" }
                    p(class = "mt-3 mb-6") { (state.about) }
                    h3(class = "text-2xl mb-2") { "Our Sponsors" }
                    (SPONSOR.widget(cx, "unsw", ()))
                    (SPONSOR.widget(cx, "ciee", ()))
                    (SPONSOR.widget(cx, "elite", ()))
                }
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
    use pulldown_cmark::{html, Parser};

    let competition_md = std::fs::read_to_string("content/competition.md")?;
    let mut competition_html = String::new();
    let md_parser = Parser::new(&competition_md);
    html::push_html(&mut competition_html, md_parser);

    let about = std::fs::read_to_string("content/about.txt")?;

    Ok(IndexState {
        competition_html,
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

use perseus::prelude::*;
use sycamore::prelude::*;
use crate::capsules::Container;

fn index_page<G: Html>(cx: Scope) -> View<G> {
    view! {
        cx,
        Container(title = "AAEO") {
            section(id = "welcome", class = "w-full h-screen bg-red-100") {

            }
            section(id = "competition", class = "w-full h-screen bg-sky-100") {

            }
            section(id = "about", class = "w-full h-screen bg-emerald-100") {

            }
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Australasian Economics Olympiad" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index").view(index_page).head(head).build()
}

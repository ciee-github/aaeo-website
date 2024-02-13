use crate::capsules::Container;
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

fn page_2023<G: Html>(cx: Scope, state: CompetitionState) -> View<G> {
    view! {
        cx,
        Container(title = "AAEO") {
            div(class = "mt-32 px-4 sm:px-6 w-full flex flex-col items-center") {
                div(class = "prose prose-slate") {
                    div(dangerously_set_inner_html = &state.content_html) {}
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, UnreactiveState, Clone)]
struct CompetitionState {
    /// The HTML for the content of the page.
    content_html: String,
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "2023 Competition | Australasian Economics Olympiad" }
    }
}

#[engine_only_fn]
async fn get_build_state(
    _: StateGeneratorInfo<()>,
) -> Result<CompetitionState, BlamedError<std::io::Error>> {
    use pulldown_cmark::{html, Parser};

    let content_md = std::fs::read_to_string("content/2023.md")?;
    let mut content_html = String::new();
    let md_parser = Parser::new(&content_md);
    html::push_html(&mut content_html, md_parser);

    Ok(CompetitionState {
        content_html,
    })
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("2023")
        .view_with_unreactive_state(page_2023)
        .head(head)
        .build_state_fn(get_build_state)
        .build()
}

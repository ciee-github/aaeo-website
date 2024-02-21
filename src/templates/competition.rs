use crate::capsules::Container;
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

fn competition_page<G: Html>(cx: Scope, state: CompetitionState) -> View<G> {
    let images_view = View::new_fragment(
        state
            .images
            .into_iter()
            .map(|(url, alt)| {
                view! {
                    cx,
                    div(class = "shrink-0 flex flex-col items-center mr-2 max-w-full relative") {
                        img(src = url, alt = alt, class = "max-h-96 object-cover") {}
                        // // Important that the caption doesn't cause overflows (so we use absolute positioning)
                        // span(class = "text-sm absolute bottom-0 left-[50%] transform -translate-x-1/2 -mb-6 overflow-ellipsis") {
                        //     (alt_text)
                        // }
                    }
                }
            })
            .collect(),
    );

    view! {
        cx,
        Container(title = "AAEO") {
            div(class = "mt-24 md:mt-32 px-4 sm:px-6 w-full flex flex-col items-center") {
                div(class = "prose prose-slate") {
                    div(dangerously_set_inner_html = &state.content_html) {}
                }
                div(class = "my-4 flex w-full overflow-x-scroll") {
                    (images_view)
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, UnreactiveState, Clone)]
struct CompetitionState {
    /// The year of the competition.
    year: u32,
    /// The HTML for the content of the page.
    content_html: String,
    /// A list of images for this competition, stored as tuples of image URLs and their alt text.
    images: Vec<(String, String)>,
}

#[engine_only_fn]
fn head(cx: Scope, state: CompetitionState) -> View<SsrNode> {
    view! { cx,
        title { (format!("{} Competition | Australasian Economics Olympiad", state.year)) }
    }
}

#[engine_only_fn]
async fn get_build_state(
    StateGeneratorInfo { path, .. }: StateGeneratorInfo<()>,
) -> Result<CompetitionState, BlamedError<std::io::Error>> {
    use pulldown_cmark::{html, Parser};

    // Parse the summary markdown
    let content_path = format!("content/{path}/summary.md");
    let content_md = std::fs::read_to_string(&content_path)?;
    let mut content_html = String::new();
    let md_parser = Parser::new(&content_md);
    html::push_html(&mut content_html, md_parser);

    // All AVIF files in the competition directory are images, filenames are alt text
    let mut images = Vec::new();
    for entry in std::fs::read_dir(&format!("content/{path}"))? {
        let entry = entry?;
        let file_name = entry.file_name();
        if let Some(file_name) = file_name.to_str() {
            if file_name.ends_with(".avif") {
                let alt_text = file_name.strip_suffix(".avif").unwrap();
                // The filename will be given a static alias
                images.push((
                    format!("static/competition-{path}-{}", file_name),
                    alt_text.to_string(),
                ));
            }
        }
    }

    Ok(CompetitionState {
        // Path is guanateed to be a number for a year
        year: path.parse().unwrap(),
        content_html,
        images,
    })
}

#[engine_only_fn]
async fn get_build_paths() -> Result<BuildPaths, std::io::Error> {
    // Past competition details are given by directories in `content/`
    let mut paths = Vec::new();
    for entry in std::fs::read_dir("content")? {
        let entry = entry?;
        // Skip files
        if !entry.file_type()?.is_dir() {
            continue;
        }
        // We only want directories with numerical names
        if let Ok(year) = entry.file_name().into_string() {
            if year.parse::<u32>().is_ok() {
                // This will be treated as a string until it's parsed in the build
                // state as a number
                paths.push(year);
            }
        }
    }

    Ok(BuildPaths {
        paths,
        extra: ().into(),
    })
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("competition")
        .view_with_unreactive_state(competition_page)
        .head_with_state(head)
        .build_paths_fn(get_build_paths)
        .build_state_fn(get_build_state)
        .build()
}

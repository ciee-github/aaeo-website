use std::path::Path;

use once_cell::sync::Lazy;
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

pub static SPONSOR: Lazy<Capsule<PerseusNodeType, ()>> = Lazy::new(|| {
    Capsule::build(
        Template::build("sponsor")
            .build_state_fn(get_build_state)
            .build_paths_fn(get_build_paths),
    )
    // TODO Skeleton
    .empty_fallback()
    .view_with_unreactive_state(sponsor_capsule)
    .build()
});

#[derive(Serialize, Deserialize, UnreactiveState, Clone)]
struct SponsorState {
    name: String,
    link: String,
    // Plain text, no formatting support.
    description: String,
    logo_url: String,
}

fn sponsor_capsule<G: Html>(cx: Scope, state: SponsorState, _props: ()) -> View<G> {
    view! {
        cx,
        // Half/half vertical split on desktop
        // One third/two thirds image/content split horizontally on mobile
        div(class = "grid grid-rows-3 md:grid-rows-1 md:grid-cols-2 max-w-5xl border-b border-slate-700 p-4") {
            div(class = "flex justify-center items-center") {
                a(href = state.link) {
                    img(src = state.logo_url, alt = state.name, height = "150", width = "150") {}
                }
            }
            div(class = "flex flex-col justify-center row-span-2 text-left") {
                (state.description)
            }
        }
    }
}

#[engine_only_fn]
async fn get_build_state(
    StateGeneratorInfo { path, .. }: StateGeneratorInfo<()>,
) -> Result<SponsorState, BlamedError<std::io::Error>> {
    let desc_path = format!("content/sponsor_{}.txt", &path);
    let raw_desc = std::fs::read_to_string(desc_path)?;
    // The first line of the description file is the link
    let mut desc_lines = raw_desc.lines().collect::<Vec<_>>();
    let link = desc_lines.remove(0).trim();
    let desc = desc_lines.join("\n");

    if !Path::new(format!("static/{}.avif", &path).as_str()).exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("logo file not found for sponsor {}", &path).as_str(),
        )
        .into());
    }

    Ok(SponsorState {
        logo_url: format!(".perseus/static/{}.png", &path),
        name: path,
        description: desc.to_string(),
        link: link.to_string(),
    })
}

#[engine_only_fn]
async fn get_build_paths() -> BuildPaths {
    BuildPaths {
        paths: vec!["unsw".to_string(), "ciee".to_string(), "elite".to_string()],
        extra: ().into(),
    }
}

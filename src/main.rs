mod templates;
mod capsules;

use perseus::prelude::*;

#[perseus::main(perseus_axum::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
        .index_view(|cx| sycamore::view! {
            cx,
            html {
                head {
                    link(rel = "stylesheet", href = ".perseus/static/tailwind.css") {}
                    meta(name = "viewport", content = "width=device-width") {}
                }
                body {
                    PerseusRoot()
                }
            }
        })
        // TODO
        .error_views(ErrorViews::unlocalized_development_default())
}

mod capsules;
mod templates;

use perseus::prelude::*;

#[perseus::main_export]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
        .capsule_ref(&*crate::capsules::SPONSOR)
        .static_alias("/favicon.ico", "static/logo.ico")
        .index_view(|cx| {
            sycamore::view! {
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
            }
        })
        // TODO
        .error_views(ErrorViews::unlocalized_development_default())
}

mod capsules;
mod global_state;
#[cfg(client)]
mod supabase;
mod templates;

use perseus::prelude::*;

#[perseus::main_export]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
        .template(crate::templates::schedule::get_template())
        .capsule_ref(&*crate::capsules::SPONSOR)
        .global_state_creator(crate::global_state::get_gsc())
        .static_alias("/favicon.ico", "static/logo.ico")
        .index_view(|cx| {
            sycamore::view! {
                cx,
                html(lang = "en") {
                    head {
                        link(rel = "stylesheet", href = ".perseus/static/tailwind.css") {}
                        meta(name = "viewport", content = "width=device-width") {}
                        meta(name = "description", content = "Information about the Australasian Economics Olympiad (AAEO).")
                        script(src = "https://unpkg.com/@supabase/supabase-js@2") {}
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

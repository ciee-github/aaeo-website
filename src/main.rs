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
        .template(crate::templates::competition::get_template())
        .template(crate::templates::schedule::get_template())
        .template(crate::templates::auth_redirect::get_template())
        .capsule_ref(&*crate::capsules::SPONSOR)
        .global_state_creator(crate::global_state::get_gsc())
        .static_alias("/favicon.ico", "static/logo.ico")
        .index_view(|cx| {
            sycamore::view! {
                cx,
                html(lang = "en") {
                    head {
                        meta(name = "viewport", content = "width=device-width") {}
                        meta(name = "description", content = "Information about the Australasian Economics Olympiad (AAEO).")

                        link(rel = "stylesheet", href = ".perseus/static/tailwind.css") {}
                        script(src = "https://unpkg.com/@supabase/supabase-js@2") {}

                        // Image preloads
                        link(rel = "preload", href = ".perseus/static/logo.ico", as = "image") {}
                        link(rel = "preload", href = ".perseus/static/welcome_bg.svg", as = "image") {}
                        link(rel = "preload", href = ".perseus/static/competition_wave.svg", as = "image") {}
                        link(rel = "preload", href = ".perseus/static/people.avif", as = "image") {}
                        link(rel = "preload", href = ".perseus/static/compass.avif", as = "image") {}
                        link(rel = "preload", href = ".perseus/static/ciee.avif", as = "image") {}
                        link(rel = "preload", href = ".perseus/static/unsw.avif", as = "image") {}
                        link(rel = "preload", href = ".perseus/static/elite.avif", as = "image") {}
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

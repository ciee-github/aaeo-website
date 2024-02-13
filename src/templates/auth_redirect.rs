use perseus::prelude::*;
use sycamore::prelude::*;

fn redirect_page<G: Html>(cx: Scope) -> View<G> {
    // Until this is populated, the redirect link will be disabled
    let redirect_url = create_signal(cx, String::new());
    let error = create_signal(cx, false);

    #[cfg(client)]
    {
        use crate::supabase::SUPABASE;
        use web_sys::UrlSearchParams;

        let location = web_sys::window().unwrap().location();
        match (|| {
            // Get the query parameters from the URL to extract the redirect target
            let query_params = location.search().ok()?;
            let query_params = UrlSearchParams::new_with_str(&query_params).ok()?;

            query_params.get("token")
        })() {
            Some(token) => {
                // Given a Supabase auth token, we can construct an appropriate URl automatically
                let redirect = format!(
                    "{}/auth/v1/verify?redirect_to=https://australasianeconolympiad.org/schedule&token={}&type=magiclink",
                    &SUPABASE.url,
                    token,
                );
                redirect_url.set(redirect.clone());
                location.replace(&redirect);
            }
            None => error.set(true),
        }
    }

    view! {
        cx,
        div(class = "flex justify-content items-center p-8 md:p-16") {
            div(class = "text-center") {
                p(class = "text-lg text-slate-400") {
                    "You should be redirected to the AAEO schedule now. If your browser does not redirect you, please "
                    a(
                        href = redirect_url,
                        disabled = redirect_url.get().is_empty()
                    ) { "click here" }
                    ". If you can't click that link just yet, wait a moment for us to resolve your login attempt."
                }
                (if *error.get() {
                    view! {
                        cx,
                        p(class = "text-red-500") { "Uh-oh, it looks like your link was malformed! Please reload the page, or double-check that this link is correct." }
                    }
                } else { View::empty() })
            }
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Authenticating... | Australasian Economics Olympiad" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("auth_redirect")
        .view(redirect_page)
        .head(head)
        .build()
}

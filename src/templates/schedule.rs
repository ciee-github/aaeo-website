use crate::capsules::Container;
use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "ScheduleStateRx")]
struct ScheduleState {
    /// The current status of the page.
    status: Status,
}

/// A representation of the private schedule data sourced from Supabase.
#[derive(Serialize, Deserialize, Clone)]
pub struct Schedule {
    /// The HTML contents of the active banner, if there is one.
    banner: String,
    /// The contents of the rest of the page, which should be fairly static.
    contents: String,
}

#[derive(Serialize, Deserialize, Clone)]
enum Status {
    /// We're either determining whether or not the user is logged in, or we're loading the actual schedule data.
    Loading,
    /// We're displaying a login prompt.
    Login,
    /// We're displaying the actual private schedule data.
    Data(Schedule),
    /// An error occurred while fetching the private schedule data. We'll only get here if the user was
    /// authenticated.
    Error,
}

#[auto_scope]
fn schedule_page<G: Html>(cx: Scope, state: &ScheduleStateRx) -> View<G> {
    #[cfg(client)]
    on_mount(cx, move || {
        use crate::supabase::SUPABASE;

        // If we already have the data, we don't need to go through this process again
        if let Status::Data(_) = &*state.status.get() {
            return;
        }

        spawn_local_scoped(cx, async {
            if SUPABASE.is_logged_in().await {
                let data = SUPABASE.fetch_schedule().await;
                match data {
                    Ok(schedule) => state.status.set(Status::Data(schedule)),
                    Err(_) => state.status.set(Status::Error),
                };
            } else {
                // We don't need to react further, as the magic link will open a new page
                state.status.set(Status::Login);
            }
        });
    });

    let email = create_signal(cx, String::new());
    let loading = create_signal(cx, false);
    let ready_for_login_link = create_signal(cx, false);
    let login_err = create_signal(cx, false);

    view! {
        cx,
        Container(title = "AAEO") {
            div(class = "flex flex-col justify-center items-center h-full w-full") {
                (match &*state.status.get() {
                    Status::Loading => view! {
                        cx,
                        Loader("Loading schedule...")
                    },
                    Status::Login => view! {
                        cx,
                        div(class = "p-12 border border-slate-400 max-w-md rounded-lg flex flex-col justify-center mx-4") {
                            h3(class = "font-bold text-xl text-center my-2") { "Login" }
                            p(class = "text-center text-slate-700 my-2") { "Please sign into your account to access protected materials for students and supervisors. If you are a contestant, this will use your school email, which will be sent an account confirmation email in the leadup to the Olympiad." }
                            input(
                                class = "border border-slate-400 focus:outline-none focus:border-yellow-400 p-2 transition-colors duration-200 rounded-lg my-2",
                                bind:value = email,
                                placeholder = "Email"
                            ) {}
                            (if *ready_for_login_link.get() {
                                view! {
                                    cx,
                                    p(class = "text-slate-400 text-sm text-center my-2") { "A magic link has been sent to your email address! Please check your inbox and click the link to sign in." }
                                }
                            } else {
                                view! {
                                    cx,
                                    button(
                                        class = format!(
                                            "bg-yellow-500 hover:bg-yellow-400 transition-colors duration-200 text-white p-2 px-16 rounded-lg my-2 {}",
                                            if *loading.get() { "animate-pulse cursor-not-allowed" } else { "" }
                                        ),
                                        disabled = *loading.get(),
                                        on:click = move |_| {
                                            // Don't even bother if the user hasn't entered an email yet
                                            if email.get().is_empty() { return }

                                            #[cfg(client)]
                                            spawn_local_scoped(cx, async move {
                                                use crate::supabase::SUPABASE;

                                                loading.set(true);
                                                match SUPABASE.sign_in(&email.get()).await {
                                                    Ok(_) => {
                                                        login_err.set(false);
                                                        ready_for_login_link.set(true);
                                                    },
                                                    Err(_) => {
                                                        login_err.set(true);
                                                        ready_for_login_link.set(false);
                                                    }
                                                };
                                                loading.set(false);
                                            })
                                        }
                                    ) {
                                        "Login"
                                    }
                                    (if *login_err.get() {
                                        view! {
                                            cx,
                                            p(class = "text-red-400 text-center text-sm my-2") { "Oops! An error occurred when trying to sign you in. Maybe try again, or make sure you're using the email address you were signed up to the Olympiad with (accounts are created by the administrators of the competition)." }
                                        }
                                    } else { View::empty() })
                                }
                            })
                        }

                    },
                    Status::Data(data) => {
                        let data = data.clone();
                        view! {
                            cx,
                            // We're dealing with content that is almost certainly longer than the page, so we need a header offset
                            div(class = "mt-32 w-full flex flex-col items-center px-4") {
                                div(
                                    class = "w-full md:w-[65ch] rounded-lg p-4 bg-yellow-500 text-yellow-900 text-center mb-12",
                                    dangerously_set_inner_html = &data.banner
                                ) {}
                                div(
                                    class = "prose",
                                    dangerously_set_inner_html = &data.contents
                                ) {}
                            }
                        }
                    },
                    Status::Error => view! {
                        cx,
                        p(class = "text-red-500") { "Uh-oh, an error occurred while fetching the schedule data! Please try reloading the page or clearing your browser cache." }
                    }
                })
            }
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! {
        cx,
        title { "AAEO Schedule" }
    }
}

#[component]
fn Loader<G: Html>(cx: Scope, msg: &'static str) -> View<G> {
    view! {
        cx,
        span(class = "font-bold text-lg") { (msg) }
        // TODO Loader icon
    }
}

#[engine_only_fn]
async fn get_build_state(_: StateGeneratorInfo<()>) -> ScheduleState {
    ScheduleState {
        // This avoids any flashes: we always start at the loader, and then go to either the schedule or the login prompt
        status: Status::Loading,
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("schedule")
        .view_with_state(schedule_page)
        .head(head)
        .build_state_fn(get_build_state)
        .build()
}

/*
 * In terms of getting this page to work properly, I want to display some basic information about the schedule of the competition to users who aren't authenticated, and
 * that can be displayed through the same API, which is good. Okay, so we take a compound vector and then parse that through, simple enough. As for the campus map
 * and detailed schedule for contestants and their managers, that should only be accessible once you log in, which means we need to fetch those data: suspense time then!
 *
 * Alright, so I'll use the suspended state system (if I can remember how to) to fetch the details we want, and then a simple reload will work once the user is logged in,
 * and that can be automatically prompted.
 *
 * In terms of layout, I'll have a two-column layout where the rightward half is devoted to public information, and the leftward asks about logging in. If the user is
 * logged in, that will display information instead. The only flash they would experience is from a login page to a loader, or, even better, we can start with the loader!
 * Yes, perfect!
 */

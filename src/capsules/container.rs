use sycamore::prelude::*;

#[component]
pub fn Container<'a, G: Html>(cx: Scope<'a>, props: ContainerProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);

    view! {
        cx,
        header(class = "w-full flex justify-center") {
            nav(class = "bg-slate-200/30 backdrop-blur-lg max-w-2xl md:max-w-4xl rounded-full mt-4 md:mt-8 p-2 px-8") {
                div(class = "w-full flex xs:justify-around items-center") {
                    a(class = "flex items-center", href = "#welcome") {
                        span(class = "bg-white rounded-full h-12 w-12 md:h-16 md:w-16 xs:ml-6") {
                            img(src = ".perseus/static/logo.ico", alt = "Logo", height = "100%", width = "100%") {}
                        }
                        h1(class = "px-4 text-xl font-bold") { (props.title) }
                    }
                    ul(class = "text-sm xs:text-base md:text-lg items-center underline hidden sm:flex") {
                        NavLink("Competition", "#competition")
                            NavLink("About", "#about")
                            NavLink("Contact", "#contact")
                        // NavLink("Schedule", "schedule")
                        // NavLink("News", "news")
                    }
                }
            }
        }
        main(class = "h-full") {
            (children)
        }
        footer(class = "bg-slate-100 w-full p-4 flex justify-center") {
            p(class = "text-sm") { "Copyright AAEO 2023-2025" }
        }
    }
}

#[allow(non_snake_case)]
fn NavLink<G: Html>(cx: Scope, title: &'static str, link: &'static str) -> View<G> {
    view! {
        cx,
        li(class = "px-2 md:px-6 link-color") {
            a(href = link) { (title) }
        }
    }
}

#[derive(Prop)]
pub struct ContainerProps<'a, G: Html> {
    title: &'static str,
    children: Children<'a, G>,
}

use sycamore::prelude::*;

#[component]
pub fn Container<'a, G: Html>(cx: Scope<'a>, props: ContainerProps<'a, G>) -> View<G> {
    let children = props.children.call(cx);

    view! {
        cx,
        header(class = "w-full bg-white") {
            nav(class = "py-4 shadow-md flex justify-center w-full px-8") {
                div(class = "w-full flex xs:justify-around items-center") {
                    a(class = "flex items-center", href = "#welcome") {
                        // TODO Logo
                        span(class = "bg-slate-500 rounded-full h-12 w-12") {}
                        h1(class = "px-4 text-xl xs:text-2xl font-bold xs:font-extrabold") { (props.title) }
                    }
                    ul(class = "sm:text-lg items-center underline hidden xs:flex") {
                        NavLink("Competition", "#competition")
                        NavLink("About", "#about")
                        NavLink("News", "news")
                    }
                }
            }
        }
        main(class = "h-full") {
            (children)
        }
        footer(class = "bg-slate-100 w-full p-4 flex justify-center") {
            p(class = "text-sm") { "Copyright AAEO 2023" }
        }
    }
}

#[allow(non_snake_case)]
fn NavLink<G: Html>(cx: Scope, title: &'static str, link: &'static str) -> View<G> {
    view! {
        cx,
        li(class = "px-2 xs:px-6") {
            a(href = link) { (title) }
        }
    }
}

#[derive(Prop)]
pub struct ContainerProps<'a, G: Html> {
    title: &'static str,
    children: Children<'a, G>
}

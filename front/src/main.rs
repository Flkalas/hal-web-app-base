use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
use pages::index_one::IndexOne;

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/1/{id}"]
    One(u32),
    #[to = "/2"]
    Two,
    #[to = "/"]
    Index,
}

pub enum Msg {
    ToggleNavbar,
}

pub struct Model {
    link: ComponentLink<Self>,
}
// #[derive(Switch, Debug, Clone)]
// pub enum ForumRoute {
//     #[to = "/{subforum}/{thread_slug}"]
//     SubForumAndThread{subforum: String, thread_slug: String}
//     #[to = "/{subforum}"]
//     SubForum{subforum: String}
// }

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleNavbar => true,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                { self.view_nav() }

                <main>
                    <Router<AppRoute, ()> render=Router::render(switch) />
                </main>
            </>
        }
    }
}

impl Model {
    fn view_nav(&self) -> Html {
        let Self { ref link, .. } = *self;

        html! {
            <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
                <div>
                    <div class="navbar-start">
                        <RouterAnchor<AppRoute> classes=classes!("navbar-item").to_string() route=AppRoute::One(12)>
                            { "One" }
                        </RouterAnchor<AppRoute>>
                        <RouterAnchor<AppRoute> classes=classes!("navbar-item").to_string() route=AppRoute::Two>
                            { "Two" }
                        </RouterAnchor<AppRoute>>
                        <RouterAnchor<AppRoute> classes=classes!("navbar-item").to_string() route=AppRoute::Index>
                            { "Index" }
                        </RouterAnchor<AppRoute>>
                    </div>
                </div>
            </nav>
        }
    }
}

fn switch(routes: &AppRoute) -> Html {
    match routes {
        AppRoute::One(id) => {
            html! { <p>{"one"} {id}</p> }
        }
        AppRoute::Two => {
            html! { <p>{"two"}</p> }
        }
        AppRoute::Index => {
            html! { <IndexOne/> }
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}

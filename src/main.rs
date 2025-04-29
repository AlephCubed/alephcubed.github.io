mod fade_in;

use crate::fade_in::FadeIn;
use yew::prelude::*;
use yew_router::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Root,
    #[at("/test")]
    Test,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Root => html! {
            <Main />
        },
        Route::Test => html! { <h1>{ "Test" }</h1> },
    }
}

#[function_component(Main)]
fn root_page() -> Html {
    html! {
        <TitleSection />
    }
}

#[function_component(TitleSection)]
fn title() -> Html {
    html! {
        <section>
            <h1 class="title">{ "AlephCubed" }</h1>
            <FadeIn>
                <img src="static/images/DefaultPlanet.png" class="planet"/>
            </FadeIn>
        </section>
    }
}

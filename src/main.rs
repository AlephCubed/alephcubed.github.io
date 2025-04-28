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
        Route::Root => html! { <h1>{ "Hello World!" }</h1> },
        Route::Test => html! { <h1>{ "Test" }</h1> },
    }
}

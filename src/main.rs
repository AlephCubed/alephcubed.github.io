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
        <div class="main">
            <TitleSection />
            <TestingSection text="More text for testing."/>
            <TestingSection text="Even more text."/>
        </div>
    }
}

#[function_component(TitleSection)]
fn title() -> Html {
    html! {
        <section>
            <h1 class="title">{ "AlephCubed" }</h1>
            <img src="static/images/DefaultPlanet.png" class="planet hidden"/>
        </section>
    }
}

#[derive(Properties, Eq, PartialEq)]
struct TextProperty { 
    pub text: String 
}

#[function_component(TestingSection)]
fn test_section(text: &TextProperty) -> Html {
    html! {
        <section class="hidden">
            <h1 class="title">{ &text.text }</h1>
        </section>
    }
}
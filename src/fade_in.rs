use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{IntersectionObserver, IntersectionObserverEntry, js_sys};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FadeInProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(FadeIn)]
pub fn fade_in_on_intersection(props: &FadeInProps) -> Html {
    let element_ref = use_node_ref();
    let is_visible = use_state(|| false);

    {
        let element_ref = element_ref.clone();
        let is_visible = is_visible.clone();

        use_effect(move || {
            let callback = Closure::wrap(Box::new(move |entries: JsValue| {
                let entries: Vec<IntersectionObserverEntry> = entries
                    .dyn_into::<js_sys::Array>()
                    .unwrap()
                    .iter()
                    .map(|entry| entry.dyn_into::<IntersectionObserverEntry>().unwrap())
                    .collect();

                for entry in entries {
                    if entry.is_intersecting() {
                        is_visible.set(true);
                    }
                }
            }) as Box<dyn Fn(JsValue)>);

            let observer = IntersectionObserver::new(callback.as_ref().unchecked_ref()).unwrap();
            if let Some(element) = element_ref.cast::<web_sys::Element>() {
                observer.observe(&element);
            }

            callback.forget();
            || {}
        });
    }

    let fade_class = if *is_visible { "fade-in" } else { "fade-out" };

    html! {
        <div ref={element_ref} class={fade_class}>
            { for props.children.iter() }
        </div>
    }
}

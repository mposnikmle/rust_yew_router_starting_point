use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Header)]
pub fn header_component() -> Html {

    html! {
        <>
            <h1>{"Hello from header!"}</h1>
        </>
    }
}
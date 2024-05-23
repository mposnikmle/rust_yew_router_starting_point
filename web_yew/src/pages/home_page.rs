use crate::components::header::Header;
use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <>
            <Header />
            <section>
                <div>
                    <p>{"Welcome to home page"}</p>
                </div>
            </section>
        </>
    }
}
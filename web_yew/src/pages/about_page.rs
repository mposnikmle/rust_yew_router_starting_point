use crate::components::header::Header;
use yew::prelude::*;

#[function_component(AboutPage)]
pub fn about_page() -> Html {
    html! {
        <>
            <Header />
            <section>
                <div>
                    <p>{"Welcome to about_page"}</p>
                </div>
            </section>
        </>
    }
}
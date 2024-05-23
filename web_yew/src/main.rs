use stylist::yew::{styled_component, Global};
use yew::prelude::*;
use yew_router::prelude::*;
mod app;
mod components;
mod pages;
mod router;
use app::App;

use log::Level;


fn main() {
    console_log::init_with_level(Level::Trace).expect("Failed to initialise Log!");
    yew::Renderer::<App>::new().render();
    }
    
    // #[derive(Clone, Routable, PartialEq)]
    // enum Route {
    //     #[at("/")]
    //     Home,
    //     #[at("/about")]
    //     About,
    //     #[at("/blog")]
    //     Blog,
    //     #[at("/signin")]
    //     SignIn,
    // }
    
    // #[styled_component]
    // pub fn App() -> Html {
    //     html! {
    //         <>
    //             // navbar
    //             <nav
    //                 class={css!(r#"
    //                     overflow: hidden;
    //                     position: fixed;
    //                     top: 0;
    //                     left: 0;
    //                     width: 100%;
    //                     display: flex;
    //                     justify-content: space-evenly;
    //                     background-color: #007A33;
    //                     padding-bottom: 10px;
    //                     font-size: 20pt;
    //                     font-family: Arial, sans-serif;
    //                 "#)}>
    //                 <a href="#">{"Home"}</a> 
    //                 <a href="#">{"About"}</a> 
    //                 <a href="#">{"Blog"}</a>
    //                 <a href="#">{"Sign in"}</a>
    //             </nav>
                
    //             // kitten image
    //             <div 
    //                 class={css!(r#"
    //                     display: flex;
    //                     justify-content: center;
    //             "#)} >
    //                 <img 
    //                 class={css!(r#"
    //                     width: 60%;
    //                     margin-top: 50px;
    //                 "#)} src="img/kittenphoto.jpg" alt="kitten photo"/>
    //             </div>
    
    //             // menu box
    //             <div
    //                 class={css!(r#"
    //                     display: flex;
    //                     justify-content: center;
    //                     justify-content: space-evenly;
    //                     background-color: green;
    
    //                     padding: 25px;
    //                     margin-top: 10px;
    //                     margin-left: 20%;
    //                     margin-right: 20%;
                        
    //                     div {
    //                         background-color: white;
    //                         padding: 25px;
    //                     }
    //                     div:hover {
    //                         background-color: cyan;
    //                     }
    //                 "#)}>
    //                 <div><h1>{"Gallery"}</h1></div>
    //                 <div><h1>{"Blog"}</h1></div>
    //                 <div><h1>{"About"}</h1></div>
    //             </div>
    //         </> 
    //     }
    // }
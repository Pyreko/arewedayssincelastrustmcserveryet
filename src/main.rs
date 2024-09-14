use leptos::{component, view, IntoView};

fn main() {
    leptos::mount_to_body(|| view! { <Application/> })
}

#[component]
fn Application() -> impl IntoView {
    view! {
        <div id="wrapper">
            <div id="main">
                <p id="status"> "No" </p>
                <div id="subtitle">
                    <p>
                        <a href="https://dayssincelastrustmcserver.com" target="_blank" rel="noopener noreferrer">"dayssincelastrustmcserver.com"</a>
                        " is "
                        <a href="https://github.com/GoldenStack/dayssincelastrustmcserver" target="_blank" rel="noopener noreferrer">"written in JavaScript"</a>
                        ", which is not Rust."</p>
                </div>
            </div>
            <div id="footer">
                <p>
                    "Site made in
                    "<a href="https://www.rust-lang.org/" target="_blank" rel="noopener noreferrer">"Rust"</a>
                    " via "
                    <a href="https://leptos.dev/" target="_blank" rel="noopener noreferrer">"Leptos"</a>" with love "
                    <a href="https://github.com/Pyreko/arewedayssincelastrustmcserveryet" target="_blank" rel="noopener noreferrer">"(GitHub)"</a>"."
                </p>
                <p>"Please note that this is all meant in good fun!"</p>
            </div>
        </div>
    }
}

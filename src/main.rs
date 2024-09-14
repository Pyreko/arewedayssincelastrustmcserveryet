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
                    <p><a href="https://dayssincelastrustmcserver.com">"dayssincelastrustmcserver.com"</a> " is "<a href="https://github.com/GoldenStack/dayssincelastrustmcserver">"written in Astro/JS"</a>", which is not Rust."</p>
                </div>
            </div>
            <div id="footer">
                <p>
                    "Site made in Rust with love "<a href="">"(GitHub)"</a>"."
                </p>
                <p>"Please note that this is all meant in good fun!"</p>
            </div>
        </div>
    }
}

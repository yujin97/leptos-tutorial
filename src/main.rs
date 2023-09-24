use leptos::*;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;

    view! {
        <button
            class:red=move || count() % 2 == 1
            on:click=move |_| { set_count.update(|n| *n += 1) }
        >

            "Click me: "
            {count}
        </button>
        <ProgressBar progress=count/>
        <ProgressBar progress=Signal::derive(double_count)/>
    }
}

/// Shows progress toward a goal.
#[component]
fn ProgressBar(
    /// The maximum value of the progress bar.
    #[prop(default = 100)]
    max: u16,
    /// How much progres should be displayed.
    #[prop(into)]
    progress: Signal<i32>,
) -> impl IntoView {
    view! { <progress max=max value=progress></progress> }
}

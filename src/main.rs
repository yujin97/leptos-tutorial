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
            on:click= move |_| {
                set_count.update(|n| *n += 1)
            }
        >
            "Click me: "
            {count}
        </button>
        <progress
            max="50"
            value=double_count
        />
        <p>
            "Double Count: "
            {double_count}
        </p>
    }
}

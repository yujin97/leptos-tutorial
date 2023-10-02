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
        <StaticViewList/>
        <DynamicItems length=10/>
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

#[component]
fn StaticViewList() -> impl IntoView {
    let values = vec![0, 1, 2];
    view! {
        <p>{values.clone()}</p>
        <ul>
            {values.into_iter()
                .map(|n| view! {<li>{n}</li>})
                .collect::<Vec<_>>()}
        </ul>
    }
}

#[component]
fn DynamicItems(length: usize) -> impl IntoView {
    let counters = (1..=length).map(|idx| create_signal(idx));

    let counter_buttons = counters
        .map(|(count, set_count)| {
            view! {
                <li>
                    <button
                        on:click= move|_| set_count.update(|n| *n += 1)
                    >
                        {count}
                    </button>
                </li>
            }
        })
        .collect_view();

    view! {
        <ul>{counter_buttons}</ul>
    }
}

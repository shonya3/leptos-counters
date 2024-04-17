use leptos::*;

#[derive(Debug, Clone)]
pub struct CounterData {
    pub id: u32,
    pub value: (ReadSignal<i32>, WriteSignal<i32>),
}

impl CounterData {
    pub fn new(id: &mut u32, value: &mut i32) -> Self {
        let counter = Self {
            id: id.to_owned(),
            value: create_signal(value.clone()),
        };
        *id += 1;
        *value += 10;
        counter
    }
}

#[component]
pub fn Counter(
    counter: ReadSignal<i32>,
    #[prop(into, optional)] on_increase_click: Option<Callback<()>>,
    #[prop(into, optional)] on_decrease_click: Option<Callback<()>>,
    #[prop(into, optional)] on_remove_click: Option<Callback<()>>,
) -> impl IntoView {
    let increase = move |_| {
        on_increase_click.as_ref().map(|f| f(()));
    };
    let decrease = move |_| {
        on_decrease_click.as_ref().map(|f| f(()));
    };
    let remove = move |_| {
        on_remove_click.as_ref().map(|f| f(()));
    };

    view! {
        <div class="counter">
            <button on:click=increase>"Increase"</button>
            <button on:click=decrease>"Decrease"</button>
            <span class="counter__label">{counter}</span>
            <button on:click=remove>"Remove"</button>
        </div>
    }
}

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let mut id = 0u32;
    let mut initial_value = 0i32;
    let (counters, set_counters) = create_signal::<Vec<CounterData>>(vec![
        CounterData::new(&mut id, &mut initial_value),
        CounterData::new(&mut id, &mut initial_value),
        CounterData::new(&mut id, &mut initial_value),
    ]);

    let mut add_counter = move || {
        set_counters
            .update(|counters| counters.push(CounterData::new(&mut id, &mut initial_value)));
    };
    let handle_click_remove = move |id: u32| {
        set_counters.update(|counters| {
            counters.retain(|c| match c.id != id {
                true => true,
                false => {
                    c.value.0.dispose();
                    false
                }
            })
        })
    };
    let handle_increase_click = move |set_count: WriteSignal<i32>| {
        set_count.update(|v| *v += 10);
    };
    let handle_decrease_click = move |set_count: WriteSignal<i32>| {
        set_count.update(|v| *v -= 10);
    };

    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            <h1>"Here"</h1>
            <button on:click=move|_| {
                add_counter();
            }>"Add counter"</button>
            <For each=counters key=|counter| counter.id children=move |counter| view! {
                <Counter counter={counter.value.0}
                    on_increase_click = move |_| {
                        handle_increase_click(counter.value.1)
                    }
                    on_remove_click = move |_| {
                        handle_click_remove(counter.id)
                    }
                    on_decrease_click = move |_| {
                        handle_decrease_click(counter.value.1)
                    }
                />
            } />
        </ErrorBoundary>
    }
}

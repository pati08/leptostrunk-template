use leptos::prelude::*;

#[component]
pub fn SimpleCounter(initial_value: i32) -> impl IntoView {
    // create a reactive signal with the initial value
    let (value, set_value) = signal(initial_value);

    // create event handlers for our buttons
    // note that `value` and `set_value` are `Copy`, so it's super easy to move them into closures
    let clear = move |_| set_value.set(0);
    let decrement = move |_| set_value.update(|value| *value -= 1);
    let increment = move |_| set_value.update(|value| *value += 1);

    const BUTTON_CLASSES: &str = "bg-gray-200 hover:bg-gray-400 active:bg-gray-600 transition-colors p-2 min-w-12 rounded shadow";

    // view! {
    //     <div class="flex flex-row items-center h-screen">
    //         <div class="flex flex-col items-center w-screen">
    //             <div class="flex flex-row gap-4 p-4">
    //                 <button on:click=clear class=BUTTON_CLASSES>Clear</button>
    //                 <button on:click=decrement class=BUTTON_CLASSES>-1</button>
    //                 // text nodes can be quoted or unquoted
    //                 <div class="flex items-center text-center">
    //                     <span>"Value: " {value} "!"</span>
    //                 </div>
    //                 <button on:click=increment class=BUTTON_CLASSES>+1</button>
    //             </div>
    //         </div>
    //     </div>
    // }
    view! {
        <div class="w-screen h-screen">
            <div class="flex flex-row gap-4 p-4 m-auto">
                <button on:click=clear class=BUTTON_CLASSES>Clear</button>
                <button on:click=decrement class=BUTTON_CLASSES>-1</button>
                // text nodes can be quoted or unquoted
                <div class="flex items-center text-center">
                    <span>"Value: " {value} "!"</span>
                </div>
                <button on:click=increment class=BUTTON_CLASSES>+1</button>
            </div>
        </div>
    }
}

// Easy to use with Trunk (trunkrs.dev) or with a simple wasm-bindgen setup
pub fn main() {
    mount_to_body(|| {
        view! {
            <SimpleCounter initial_value=3 />
        }
    })
}

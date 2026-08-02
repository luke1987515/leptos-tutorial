use leptos::mount::mount_to_body;
use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    // 1. 建立 count 狀態，初始值為 0
    let (count, set_count) = signal(0);

    view! {
        <p>"Hello, world!"</p>
        <button
            on:click=move |_| {
                // 修改狀態：加 1 (也可以用 set_count.set(3);)
                *set_count.write() += 1;
            }
        >
            "Click me: "
            // 讀取狀態：傳遞 Signal 本身或閉包，才能觸發動態更新
            {count}
        </button>
        <p>
            "Double count: "
            {move || count.get() * 2}
        </p>
    }
}

fn main() {
    mount_to_body(App);
}
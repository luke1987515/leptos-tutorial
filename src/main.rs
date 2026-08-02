use leptos::html;
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

#[component]
fn App() -> impl IntoView {
    // 1. 建立 count 狀態，初始值為 0
    let (count, set_count) = signal(0);

    view! {
        <p>"Hello, world!"</p>
        <button
            on:click=move |_| {
                *set_count.write() += 1;
            }
        >
            "Click me: "
            {count}
        </button>
        <p>
            "Double count: "
            {move || count.get() * 2}
        </p>

        <hr style="margin: 2rem 0; border-color: #334155;" />

        <Home />
    }
}

#[component]
pub fn Home() -> impl IntoView {
    let (max_iterations, set_max_iterations) = signal(300i32);
    let (elapsed_time, set_elapsed_time) = signal(0.0f64);
    let (calculated_pixels, set_calculated_pixels) = signal(0i64);

    // 修正：使用 NodeRef::<html::Canvas>::new()
    let canvas_ref = NodeRef::<html::Canvas>::new();

    let render_mandelbrot = move |iter: i32| {
        if let Some(canvas) = canvas_ref.get() {
            let ctx = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .unchecked_into::<web_sys::CanvasRenderingContext2d>();

            let width = canvas.width() as usize;
            let height = canvas.height() as usize;

            let mut pixels = vec![0u8; width * height * 4];
            let start = js_sys::Date::now();
            let mut total_calculations: i64 = 0;

            for y in 0..height {
                for x in 0..width {
                    let cx = -2.0 + (x as f64 / width as f64) * 3.0;
                    let cy = -1.5 + (y as f64 / height as f64) * 3.0;

                    let mut zx = 0.0;
                    let mut zy = 0.0;
                    let mut i = 0;

                    while zx * zx + zy * zy <= 4.0 && i < iter {
                        let temp = zx * zx - zy * zy + cx;
                        zy = 2.0 * zx * zy + cy;
                        zx = temp;
                        i += 1;
                    }
                    total_calculations += i as i64;

                    let idx = (y * width + x) * 4;
                    if i == iter {
                        pixels[idx] = 15;
                        pixels[idx + 1] = 23;
                        pixels[idx + 2] = 42;
                        pixels[idx + 3] = 255;
                    } else {
                        let ratio = i as f64 / iter as f64;
                        pixels[idx] = (124.0 * (1.0 - ratio) + 6.0 * ratio) as u8;
                        pixels[idx + 1] = (58.0 * (1.0 - ratio) + 182.0 * ratio) as u8;
                        pixels[idx + 2] = (237.0 * (1.0 - ratio) + 212.0 * ratio) as u8;
                        pixels[idx + 3] = 255;
                    }
                }
            }

            let end = js_sys::Date::now();
            set_elapsed_time.set(end - start);
            set_calculated_pixels.set(total_calculations);

            let image_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
                wasm_bindgen::Clamped(&mut pixels),
                canvas.width(),
                canvas.height(),
            )
            .unwrap();

            ctx.put_image_data(&image_data, 0.0, 0.0).unwrap();
        }
    };

    Effect::new(move |_| {
        let iter = max_iterations.get();
        render_mandelbrot(iter);
    });

    view! {
        <div style="padding: 2rem; max-width: 900px; margin: 0 auto; color: #fff; background-color: #0f172a; font-family: sans-serif;">
            <section class="hero-section" style="margin-bottom: 2rem;">
                <h1 class="hero-title" style="font-size: 2rem; line-height: 1.3;">
                    "極速 WASM 驅動的" <br />
                    <span style="color: #06b6d4;">"離線優先 Web 應用"</span>
                </h1>
                <p class="hero-subtitle" style="color: #94a3b8;">
                    "利用 Rust + Leptos CSR 建構，體驗媲美原生應用的極致流暢感。"
                </p>
            </section>

            <div class="glass-card" style="background: rgba(30, 41, 59, 0.7); border-radius: 1rem; padding: 1.5rem; border: 1px solid #334155;">
                <div class="benchmark-container">
                    <div class="benchmark-header">
                        <h2 style="font-size: 1.5rem; font-weight: 800; margin-bottom: 0.5rem; color: #38bdf8;">
                            "🦀 WASM 高密度運算基準測試"
                        </h2>
                        <p style="color: #94a3b8; font-size: 0.95rem;">
                            "此測試使用 Rust 直接在您的瀏覽器端渲染 Mandelbrot (曼德博) 碎形圖。拉動下方的滑桿調整最大疊代次數，體驗 WebAssembly 即時計算的驚人效能！"
                        </p>
                    </div>

                    <div class="benchmark-grid" style="display: flex; gap: 2rem; flex-wrap: wrap; margin-top: 1rem;">
                        <div class="canvas-container">
                            <canvas
                                node_ref=canvas_ref
                                width="256"
                                height="256"
                                style="border-radius: 0.5rem; border: 1px solid #475569;"
                            ></canvas>
                        </div>

                        <div class="controls-panel" style="flex: 1; min-width: 250px; display: flex; flex-direction: column; gap: 1rem;">
                            <div class="slider-group">
                                <label for="iterations" style="display: flex; justify-content: space-between; margin-bottom: 0.5rem;">
                                    <span>"最大迭代次數 (Iterations)"</span>
                                    <span style="color: #06b6d4; font-weight: 700;">{move || max_iterations.get()}</span>
                                </label>
                                <input
                                    type="range"
                                    id="iterations"
                                    min="50"
                                    max="1000"
                                    step="50"
                                    prop:value=move || max_iterations.get().to_string()
                                    on:input=move |e| {
                                        // 修正：直接呼叫 event_target_value(&e)
                                        if let Ok(val) = event_target_value(&e).parse::<i32>() {
                                            set_max_iterations.set(val);
                                        }
                                    }
                                    style="width: 100%; cursor: pointer;"
                                />
                            </div>

                            <div class="stats-card" style="display: flex; gap: 1rem; background: #0f172a; padding: 1rem; border-radius: 0.5rem;">
                                <div class="stat-item" style="flex: 1;">
                                    <div style="font-size: 1.25rem; font-weight: bold; color: #38bdf8;">
                                        {move || format!("{:.1}", elapsed_time.get())} " ms"
                                    </div>
                                    <div style="font-size: 0.8rem; color: #64748b;">"運算耗時"</div>
                                </div>
                                <div class="stat-item" style="flex: 1;">
                                    <div style="font-size: 1.25rem; font-weight: bold; color: #a855f7;">
                                        {move || {
                                            let count = calculated_pixels.get();
                                            if count >= 1_000_000 {
                                                format!("{:.1}M", count as f64 / 1_000_000.0)
                                            } else {
                                                format!("{:.1}K", count as f64 / 1000.0)
                                            }
                                        }}
                                    </div>
                                    <div style="font-size: 0.8rem; color: #64748b;">"總迴圈計算次數"</div>
                                </div>
                            </div>

                            <button
                                style="padding: 0.6rem; background: #0284c7; color: white; border: none; border-radius: 0.5rem; cursor: pointer; font-weight: bold;"
                                on:click=move |_| render_mandelbrot(max_iterations.get())
                            >
                                "重新計算"
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
mod api;
mod store;
mod components;
mod router;
mod pages;
mod app;


fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<app::App>::new().render();
}
mod api;
mod store;
mod components;
mod router;
mod pages;
mod app;


fn main() {
    yew::Renderer::<app::App>::new().render();
}
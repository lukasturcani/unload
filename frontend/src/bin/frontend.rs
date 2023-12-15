use frontend::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(App);
}

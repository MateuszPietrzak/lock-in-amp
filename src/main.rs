mod app;
mod signal_plot;
mod parameter_input;
mod input_signal_settings;
mod input_signal_gen;
mod lock_in_amplifier;

use crate::app::App;

fn main() -> iced::Result {
    #[cfg(target_arch = "wasm32")]
    {
        console_log::init().expect("Initialize logger");
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    }

    #[cfg(not(target_arch = "wasm32"))]
    tracing_subscriber::fmt::init();

    iced::application(App::default, App::update, App::view)
        .title("Lock In Amplifier")
        .antialiasing(cfg!(not(target_arch = "wasm32")))
        .centered()
        .run()
}


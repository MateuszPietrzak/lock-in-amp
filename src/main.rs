use iced::{
    Element, Length::Fill, widget::{button, column, container, text, text_input}
};

pub fn main() -> iced::Result {
    #[cfg(target_arch = "wasm32")]
    {
        console_log::init().expect("Initialize logger");
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    }

    #[cfg(not(target_arch = "wasm32"))]
    tracing_subscriber::fmt::init();

    iced::application(App::default, App::update, App::view)
        .title("Lock In Amp")
        .centered()
        .run()
}

#[derive(Default)]
struct App {
    value: i64,
    text: String,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
    TextContentChanged(String),
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
            Message::TextContentChanged(str) => {
                self.text = str;
            },
        }
    }

    fn view(&self) -> Element<'_, Message> {
        container(
            column![
                button("+").on_press(Message::Increment),
                text(self.value),
                button("-").on_press(Message::Decrement),
                text_input("Type something here", &self.text).on_input(Message::TextContentChanged),
            ]
            .spacing(10),
        )
        .padding(10)
        .into()
    }
}

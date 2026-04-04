use iced::{
    Element,
    widget::{button, column, container, text, text_input},
};
use iced_core::Length;
use plotters::{
    prelude::PathElement,
    series::LineSeries,
    style::{BLACK, Color, RED, WHITE},
};
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};

pub fn main() -> iced::Result {
    #[cfg(target_arch = "wasm32")]
    {
        console_log::init().expect("Initialize logger");
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    }

    #[cfg(not(target_arch = "wasm32"))]
    tracing_subscriber::fmt::init();

    iced::application("Lock In Amp", App::update, App::view)
        .antialiasing(cfg!(not(target_arch = "wasm32")))
        .centered()
        .run()
}

#[derive(Default)]
struct App {
    value: i64,
    text: String,
    chart: MyChart,
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
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        container(column![
            button("+").on_press(Message::Increment),
            text(self.value),
            button("-").on_press(Message::Decrement),
            text_input("Type something here...", &self.text).on_input(Message::TextContentChanged),
            ChartWidget::new(&self.chart)
                .width(Length::Fixed(600.))
                .height(Length::Fixed(400.0))
        ].spacing(10.))
        .padding(10)
        .into()
    }
}

#[derive(Default)]
struct MyChart;

impl Chart<Message> for MyChart {
    type State = ();

    fn build_chart<DB: DrawingBackend>(&self, _state: &Self::State, mut builder: ChartBuilder<DB>) {
        let mut chart = builder
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        chart
            .draw_series(LineSeries::new(
                (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
                &RED,
            ))
            .unwrap()
            .label("y = x^2")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()
            .unwrap();
    }
}

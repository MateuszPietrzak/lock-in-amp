use iced::widget::button;
use iced::widget::column;
use iced::widget::text;
use iced_plot::PlotUiMessage;
use iced_plot::PlotWidget;
use iced_plot::Series;
use iced_plot::{MarkerStyle, PlotWidgetBuilder};

use iced::{Color, Element};

pub fn main() -> iced::Result {
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

struct App {
    value: i64,
    plot_widget: PlotWidget,
}

impl Default for App {
    fn default() -> Self {
        Self {
            value: 0,
            plot_widget: build_plot(),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
    PlotMessage(PlotUiMessage),
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
            Message::PlotMessage(plot_ui_message) => self.plot_widget.update(plot_ui_message),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let plot_widget = self.plot_widget.view().map(Message::PlotMessage);

        column![
            button("+").on_press(Message::Increment),
            text(self.value),
            button("-").on_press(Message::Decrement),
            plot_widget,
        ].spacing(10).padding(10).into()
    }
}

fn build_plot() -> PlotWidget {
    let positions = (0..10)
        .map(|x| [x as f64, (x * x) as f64])
        .collect::<Vec<[f64; 2]>>();

    let series = Series::markers_only(positions, MarkerStyle::circle(1.0))
        .with_label("2d Gaussian scatter - 5M points")
        .with_color(Color::from_rgb(0.2, 0.6, 1.0));

    PlotWidgetBuilder::new()
        .add_series(series.clone())
        .build()
        .unwrap()
}

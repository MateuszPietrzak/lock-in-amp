use iced_plot::LineStyle;
use iced_plot::PlotUiMessage;
use iced_plot::PlotWidget;
use iced_plot::PlotWidgetBuilder;
use iced_plot::Series;
use iced_plot::ShapeId;
use std::sync::Arc;


use iced::{Color, Element};

pub struct SignalPlot {
    series_id: ShapeId,
    plot_widget: PlotWidget,
}

impl SignalPlot {
    pub fn new() -> Self {
        let positions = vec![[0.0, 0.0]];
        // ^ for some reason PlotWidget blows up with empty series so you know... {0.0, 0.0} point 

        let series = Series::line_only(positions, LineStyle::Solid)
            .with_color(Color::from_rgb(0.2, 0.6, 1.0));

        let plot_widget = PlotWidgetBuilder::new()
            .add_series(series.clone())
            .build()
            .unwrap();

        Self {
            series_id: series.id,
            plot_widget,
        }
    }

    pub fn set_series(&mut self, signal: Arc<Vec<f32>>, time_axis: Arc<Vec<f32>>) {

        let new_series : Vec<[f64; 2]> = time_axis
        .iter()
        .zip(signal.iter())
        .map(|(&x, &y)| [x as f64, y as f64])
        .collect();

        self.plot_widget
            .update_series(&self.series_id, |s| s.positions = new_series.clone())
            .unwrap();
    }

    pub fn update(&mut self, message: PlotUiMessage) {
        self.plot_widget.update(message);
    }

    pub fn view(&self) -> Element<PlotUiMessage> {
        self.plot_widget.view()
    }
}

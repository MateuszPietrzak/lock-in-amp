use iced_plot::LineStyle;
use iced_plot::PlotUiMessage;
use iced_plot::PlotWidget;
use iced_plot::PlotWidgetBuilder;
use iced_plot::Series;
use iced_plot::ShapeId;

use iced::{Color, Element};

pub struct InputSignalPlot {
    series_id: ShapeId,
    plot_widget: PlotWidget,
    points: u64,
}

impl InputSignalPlot {
    pub fn new() -> Self {
        let positions = (0..500)
            .map(|i| {
                let x = (i as f64) / 30.;
                [x, x.sin()]
            })
            .collect::<Vec<[f64; 2]>>();

        let series = Series::line_only(positions, LineStyle::Solid)
            .with_color(Color::from_rgb(0.2, 0.6, 1.0));

        let plot_widget = PlotWidgetBuilder::new()
            .add_series(series.clone())
            .build()
            .unwrap();

        Self {
            series_id: series.id,
            plot_widget,
            points: 500,
        }
    }

    pub fn add_more_points(&mut self) {
        self.plot_widget
            .update_series(&self.series_id, |series| {
                for i in self.points..(self.points + 50) {
                    let x = (i as f64) / 30.;
                    series.positions.push([x, x.sin()]);
                }

                self.points += 50;
            })
            .unwrap();
    }

    pub fn update(&mut self, message: PlotUiMessage) {
        self.plot_widget.update(message);
    }

    pub fn view(&self) -> Element<'_, PlotUiMessage> {
        self.plot_widget.view()
    }
}

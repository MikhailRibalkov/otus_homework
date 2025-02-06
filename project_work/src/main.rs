use iced::time;
use iced::{Element, Settings, Subscription, Task};
use plotters::prelude::*;
use plotters_iced::{Chart, ChartWidget, DrawingBackend};
use rand::Rng;
use std::time::Duration;

#[derive(Debug, Clone)]
enum Message {
    Tick,
}
#[derive(Default, Debug)]
struct RealTimeChart {
    data: Vec<(f32, f32)>,
}

impl RealTimeChart {
    fn new(_flags: ()) -> (Self, Task<Message>) {
        (Self { data: vec![] }, Task::none())
    }

    fn title(&self) -> String {
        String::from("Real-Time Chart Example")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                let mut rng = rand::thread_rng();
                let new_value = rng.gen_range(0.0..100.0);
                let time = self.data.len() as f32;
                self.data.push((time, new_value));

                if self.data.len() > 50 {
                    self.data.remove(0); // Удаление старых данных
                }
            }
        }
        Task::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_millis(500)).map(|_| Message::Tick)
    }

    fn view(&self) -> Element<Message> {
        ChartWidget::new(self).into()
    }
}

impl Chart<Message> for RealTimeChart {
    type State = ();
    fn build_chart<DB: DrawingBackend>(&self, _: &Self::State, builder: ChartBuilder<DB>) {
        let mut builder = builder;
        let mut chart = builder
            .caption("Real-Time Data", ("sans-serif", 30))
            .x_label_area_size(30)
            .y_label_area_size(40)
            .build_cartesian_2d(0f32..50f32, 0f32..100f32)
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        chart
            .draw_series(LineSeries::new(self.data.iter().cloned(), &RED))
            .unwrap();
    }
}

fn main() -> iced::Result {
    iced::run("charts", RealTimeChart::update, RealTimeChart::view)
}

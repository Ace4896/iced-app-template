use std::time::Duration;

#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

#[cfg(target_arch = "wasm32")]
use wasm_timer::Instant;

use iced::{
    alignment, executor, theme, time, widget, Application, Command, Element, Length, Subscription,
    Theme,
};

pub struct App {
    duration: Duration,
    state: State,
}

pub enum State {
    Idle,
    Ticking { last_tick: Instant },
}

#[derive(Clone, Debug)]
pub enum Message {
    Toggle,
    Reset,
    Tick(Instant),
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            App {
                duration: Duration::default(),
                state: State::Idle,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Stopwatch - Iced")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Toggle => match self.state {
                State::Idle => {
                    self.state = State::Ticking {
                        last_tick: Instant::now(),
                    };
                }
                State::Ticking { .. } => self.state = State::Idle,
            },
            Message::Reset => self.duration = Duration::default(),
            Message::Tick(now) => {
                if let State::Ticking { last_tick } = &mut self.state {
                    self.duration += now - *last_tick;
                    *last_tick = now;
                }
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        match self.state {
            State::Idle => Subscription::none(),
            State::Ticking { .. } => {
                time::every(Duration::from_millis(10)).map(|i| Message::Tick(i))
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        const MINUTE: u64 = 60;
        const HOUR: u64 = 60 * MINUTE;

        let seconds = self.duration.as_secs();

        let duration = widget::text(format!(
            "{:0>2}:{:0>2}:{:0>2}.{:0>2}",
            seconds / HOUR,
            (seconds % HOUR) / MINUTE,
            seconds % MINUTE,
            self.duration.subsec_millis() / 10,
        ))
        .size(40);

        let button = |label| {
            widget::button(widget::text(label).horizontal_alignment(alignment::Horizontal::Center))
                .padding(10)
                .width(80)
        };

        let toggle_button = {
            let label = match self.state {
                State::Idle => "Start",
                State::Ticking { .. } => "Stop",
            };

            button(label).on_press(Message::Toggle)
        };

        let reset_button = button("Reset")
            .style(theme::Button::Destructive)
            .on_press(Message::Reset);

        let controls = widget::row![toggle_button, reset_button].spacing(20);

        let content = widget::column![duration, controls]
            .align_items(alignment::Alignment::Center)
            .spacing(20);

        widget::container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

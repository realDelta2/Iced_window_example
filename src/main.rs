use iced::{Application, Settings};
use iced::widget::{button, text, container, column};
use iced::window;
use iced::{Element, Renderer, Theme, Command, executor, Alignment, Length};

const DEFAULT_WINDOW_SIZE: (u32, u32) = (220, 220);
fn main() {
   let _ = IcyWindow::run(Settings { window: {
        window::Settings { size: DEFAULT_WINDOW_SIZE, 
            ..Default::default()
         }
    },
    ..Settings::default()}
    );
}

struct IcyWindow {
    x: u32,
    y: u32
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Shrink,
    Expand
}

impl  Application for IcyWindow {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn title(&self) -> String {
        String::from("Icy Window Resizing")
    }

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (IcyWindow {x: DEFAULT_WINDOW_SIZE.0, y: DEFAULT_WINDOW_SIZE.1}, Command::none())
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Shrink => {
                self.x -= 10;
                self.y -= 10;
                window::resize(self.x, self.y)
            },
            Message::Expand => {
                self.x += 10;
                self.y += 10;
                window::resize(self.x, self.y)
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message, crate::Renderer<Self::Theme>> {
        container(
            column!(
                button("expand").on_press(Message::Expand),
                text(format!("x:{}, y:{}", self.x, self.y)),
                button("shrink").on_press(Message::Shrink)
            ).spacing(5).align_items(Alignment::Center)
        ).center_x().center_y().width(Length::Fill)
        .height(Length::Fill).into()
    }
}
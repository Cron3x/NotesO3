mod loaders;
mod ui;
mod theme;

use iced::{
    theme::{self, Theme},
    Color,
    executor, Application, Command, Element, Settings,
    widget::{
        Text, Column, Space, Row, Container
    }, Length, Alignment, alignment::Horizontal, Padding, Pixels, Renderer
};

fn main() -> iced::Result {
    //loaders::load_files();
    App::run( Settings::default())
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Route {
    Notes,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Message {
    FetchedFS(bool),
    FileDialog,
    StyleBarToggle,
}

struct App {
    theme: Theme,
    route: Route,
}

impl Application for App {
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;

    fn title(&self) -> String{
        "cyprs - Notes".to_string()
    }

    fn new(_flags: ()) -> (App, Command<Message>) {
        (
            App{
                theme: Theme::custom(
                        theme::Palette {
                            background: Color::from_rgb8(30,30,46),
                            text: Color::from_rgb8(205,214,244),
                            primary: Color::from_rgb8(17, 17, 27),
                            success: Color::from_rgb(0.0, 1.0, 0.0),
                            danger: Color::from_rgb(1.0, 0.0, 0.0),
                        }
                    ),
                route: Route::Notes,
            },
            Command::perform(loaders::load_files(), Message::FetchedFS),
        )
    }

    fn theme(&self) -> Theme{
        self.theme.clone()
    }

    fn view(&self) -> Element<Message>{
        let r = Row::new()
            .padding(Padding::from(10))
            .push(Text::new(format!("File")).size(20))
            .push(Text::new(format!("◉◉◉")).size(20));
        Container::new(r)
            .width(Length::Fill)
            .style(theme::Container::Box)
        .into()
    }

    fn update(&mut self, msg: Message) -> Command<Message> {
        match msg {
            Message::FetchedFS(_) => {
            }
            _ => {}
        }
        Command::none()
    }
}

struct Topbar {}

struct Canvas {}

struct Sidebar {}

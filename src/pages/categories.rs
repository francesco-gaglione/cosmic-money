use cosmic::{widget, Command, Element};

#[derive(Debug, Clone)]
pub enum CategoriesMessage {
    TempMessage,
}

pub struct Categories {}

impl Default for Categories {
    fn default() -> Self {
        Self {}
    }
}

impl Categories {
    pub fn view<'a>(&self) -> Element<'a, CategoriesMessage> {
        widget::text("cat page").into()
    }

    pub fn update(&mut self, message: CategoriesMessage) -> Command<crate::app::Message> {
        match message {
            CategoriesMessage::TempMessage => {
                println!("temp message");
            }
        }
        Command::none()
    }
}

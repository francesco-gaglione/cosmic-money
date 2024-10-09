use cosmic::{
    iced::{Alignment, Length},
    widget, Command, Element,
};

use crate::{
    fl,
    models::{Category, NewCategory},
    STORE,
};

#[derive(Debug, Clone)]
pub enum CategoriesMessage {
    AddCategory,
    NewCategoryNameChanged(String),
    NewCategorySubmitted,
}

pub struct Categories {
    categories: Vec<Category>,
    add_category_view: bool,
    form_new_category_name: String,
}

impl Default for Categories {
    fn default() -> Self {
        let mut store = STORE.lock().unwrap();
        let categories = store.get_categories();
        Self {
            categories: if let Ok(cat) = categories {
                cat
            } else {
                vec![]
            },
            add_category_view: false,
            form_new_category_name: "".to_string(),
        }
    }
}

impl Categories {
    pub fn view<'a>(&'a self) -> Element<'a, CategoriesMessage> {
        let mut element = widget::column()
            .width(Length::Fill)
            .align_items(Alignment::Start);

        element = element.push(
            widget::row()
                .push(
                    widget::column()
                        .push(widget::text::title1(fl!("page_categories")))
                        .width(Length::Fill),
                )
                .push(
                    widget::column()
                        .push(
                            widget::row()
                                .push(
                                    widget::button::text(fl!("view-settings"))
                                        .style(widget::button::Style::Standard),
                                )
                                .push(widget::horizontal_space(5))
                                .push(
                                    widget::button::text(fl!("add-category"))
                                        .on_press(CategoriesMessage::AddCategory)
                                        .style(widget::button::Style::Suggested),
                                ),
                        )
                        .width(Length::Fill)
                        .align_items(Alignment::End),
                ),
        );

        if self.add_category_view {
            element = element.push(widget::vertical_space(Length::from(10)));
            element = element.push(
                widget::container(
                    widget::column()
                        .push(widget::text::title4(fl!("new-category")))
                        .push(widget::vertical_space(Length::from(10)))
                        .push(
                            widget::row().push(
                                widget::column()
                                    .push(widget::text::text(fl!("category-name")))
                                    .push(widget::vertical_space(Length::from(3)))
                                    .push(
                                        cosmic::widget::text_input(
                                            fl!("new-category"),
                                            &self.form_new_category_name,
                                        )
                                        .on_input(CategoriesMessage::NewCategoryNameChanged),
                                    ),
                            ),
                        )
                        .push(widget::vertical_space(Length::from(10)))
                        .push(
                            widget::row()
                                .push(
                                    widget::button::text(fl!("add-category"))
                                        .on_press(CategoriesMessage::NewCategorySubmitted)
                                        .style(widget::button::Style::Suggested),
                                )
                                .width(Length::Fill)
                                .align_items(Alignment::End),
                        )
                        .width(Length::Fill),
                )
                .padding(10)
                .width(Length::Fill)
                .style(cosmic::theme::Container::Card),
            );

            element = element.push(widget::vertical_space(Length::from(10)));
        }

        element = element.push(widget::vertical_space(Length::from(10)));

        log::info!("categories: {:?}", self.categories);

        for c in &self.categories {
            log::info!("creating: {:?}", c.name);
            element = element
                .push(
                    widget::container(
                        widget::row()
                            .push(
                                widget::column()
                                    .push(widget::text::title4(c.name.clone()))
                                    .width(Length::Fill),
                            )
                            .push(
                                widget::column()
                                    .push(widget::text::text("calculate$"))
                                    .align_items(Alignment::End)
                                    .width(Length::Fill),
                            ),
                    )
                    .padding(10)
                    .style(cosmic::theme::Container::Card),
                )
                .push(widget::vertical_space(Length::from(10)));
        }

        element.into()
    }

    pub fn update(&mut self, message: CategoriesMessage) -> Command<crate::app::Message> {
        match message {
            CategoriesMessage::AddCategory => {
                self.add_category_view = true;
            }
            CategoriesMessage::NewCategoryNameChanged(value) => {
                self.form_new_category_name = value;
            }
            CategoriesMessage::NewCategorySubmitted => {
                let new_category = NewCategory {
                    name: self.form_new_category_name.as_str(),
                };
                let mut store = STORE.lock().unwrap();
                store.create_category(&new_category);
                self.add_category_view = false;
                self.form_new_category_name = "".to_string();
                //TODO update categories
            }
        }
        Command::none()
    }
}

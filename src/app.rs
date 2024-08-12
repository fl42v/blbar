use crate::{
    config::{self, Config},
    get_log_spec,
    menu::{menu_wrapper, Menu, MenuType},
    modules::{
        clock::Clock, launcher, privacy::Privacy, settings::Settings, system_info::SystemInfo,
        title::Title, updates::Updates, workspaces::Workspaces,
    },
    style::ashell_theme,
    HEIGHT,
};
use flexi_logger::LoggerHandle;
use iced::{
    widget::{column, container, row, Column, Row, Space},
    window::Id,
    Alignment, Application, Color, Length, Theme,
};

use crate::style::header_pills;

pub struct App {
    //logger: LoggerHandle,
    config: Config,
    menu: Menu,
    updates: Updates,
    workspaces: Workspaces,
    window_title: Title,
    system_info: SystemInfo,
    clock: Clock,
    privacy: Privacy,
    pub settings: Settings,
}

#[derive(Debug, Clone)]
pub enum Message {
    None,
    ConfigChanged(Box<Config>),
    CloseMenu,
    OpenLauncher,
    Updates(crate::modules::updates::Message),
    Workspaces(crate::modules::workspaces::Message),
    Title(crate::modules::title::Message),
    SystemInfo(crate::modules::system_info::Message),
    Clock(crate::modules::clock::Message),
    Privacy(crate::modules::privacy::PrivacyMessage),
    Settings(crate::modules::settings::Message),
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Theme = Theme;
    type Message = Message;
    type Flags = (Config);

    fn new((config): (Config)) -> (Self, iced::Command<Self::Message>) {
        (
            App {
                //logger,
                config,
                menu: Menu::init(),
                updates: Updates::new(),
                workspaces: Workspaces::new(),
                window_title: Title::new(),
                system_info: SystemInfo::new(),
                clock: Clock::new(),
                privacy: Privacy::new(),
                settings: Settings::new(),
            },
            iced::Command::none(),
        )
    }

    fn theme(&self, _id: Id) -> Self::Theme {
        ashell_theme(&self.config.appearance)
    }

    fn style(&self) -> iced::theme::Application {
        fn dark_background(theme: &Theme) -> iced::wayland::Appearance {
            iced::wayland::Appearance {
                background_color: Color::TRANSPARENT,
                //background_color: Color::WHITE,
                text_color: theme.palette().text,
                icon_color: theme.palette().text,
            }
        }

        iced::theme::Application::from(dark_background as fn(&Theme) -> _)
    }

    fn title(&self, _id: Id) -> String {
        String::from("ashell")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::None => iced::Command::none(),
            Message::ConfigChanged(config) => {
                log::info!("New config: {:?}", config);
                self.config = *config;
                //self.logger
                //    .set_new_spec(get_log_spec(self.config.log_level));
                iced::Command::none()
            }
            Message::CloseMenu => self.menu.close(),
            Message::Updates(message) => {
                if let Some(updates_config) = self.config.updates.as_ref() {
                    self.updates
                        .update(message, updates_config, &mut self.menu)
                        .map(Message::Updates)
                } else {
                    iced::Command::none()
                }
            }
            Message::OpenLauncher => {
                if let Some(app_launcher_cmd) = self.config.app_launcher_cmd.as_ref() {
                    crate::utils::launcher::execute_command(app_launcher_cmd.to_string());
                }
                iced::Command::none()
            }
            Message::Workspaces(msg) => {
                self.workspaces.update(msg);

                iced::Command::none()
            }
            Message::Title(message) => {
                self.window_title
                    .update(message, self.config.truncate_title_after_length);
                iced::Command::none()
            }
            Message::SystemInfo(message) => {
                self.system_info.update(message);
                iced::Command::none()
            }
            Message::Clock(message) => {
                self.clock.update(message);
                iced::Command::none()
            }
            Message::Privacy(message) => self
                .privacy
                .update(message, &mut self.menu)
                .map(Message::Privacy),
            Message::Settings(message) => self
                .settings
                .update(message, &self.config.settings, &mut self.menu)
                .map(Message::Settings),
        }
    }

    fn view(&self, id: Id) -> iced::Element<'_, Self::Message> {
        if Some(id) == self.menu.get_id() {
            if let Some(menu_type) = self.menu.get_menu_type() {
                menu_wrapper(
                    match menu_type {
                        MenuType::Updates => self.updates.menu_view().map(Message::Updates),
                        MenuType::Privacy => self.privacy.menu_view().map(Message::Privacy),
                        MenuType::Settings => self
                            .settings
                            .menu_view(&self.config.settings)
                            .map(Message::Settings),
                    },
                    match menu_type {
                        MenuType::Updates => crate::menu::MenuPosition::Left,
                        MenuType::Privacy => crate::menu::MenuPosition::Right,
                        MenuType::Settings => crate::menu::MenuPosition::Left,
                    },
                )
            } else {
                row!().into()
            }
        } else {
            let mut left = column!().spacing(4).padding([8, 8, 8, 12]);
            if let Some(sysinfo) = self.system_info.view(&self.config.system) {
                left = left.push(sysinfo.map(Message::SystemInfo));
            }

            let mut center = column!().spacing(4).padding(8);
            center = center.push(
                self.workspaces
                    .view(&self.config.appearance.workspace_colors)
                    .map(Message::Workspaces),
            );

            let right = Column::with_children(
                vec![Some(
                    Column::with_children(
                        vec![
                            Some(
                                self.clock
                                    .view(&self.config.clock.format)
                                    .map(Message::Clock),
                            ),
                            if self.privacy.applications.is_empty() {
                                None
                            } else {
                                Some(self.privacy.view().map(Message::Privacy))
                            },
                            Some(self.settings.view().map(Message::Settings)),
                        ]
                        .into_iter()
                        .flatten()
                        .collect::<Vec<_>>(),
                    )
                    .padding(8)
                    .into(),
                )]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>(),
            )
            .align_items(iced::Alignment::Center)
            .spacing(4);

            container(
                container(
                    column![
                        //container(left).style(header_pills),
                        //Space::with_height(Length::Fill),
                        //container(center).style(header_pills),
                        //Space::with_height(Length::Fill),
                        //container(right).style(header_pills),
                        left,
                        Space::with_height(Length::Fill),
                        center,
                        Space::with_height(Length::Fill),
                        right,
                    ]
                    .align_items(iced::Alignment::Center),
                )
                .padding([5, 0, 5, 0])
                .style(header_pills),
            )
            .padding([10, 0, 10, 10])
            .into()
        }
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::Subscription::batch(
            vec![
                self.config.updates.as_ref().map(|updates_config| {
                    self.updates
                        .subscription(updates_config)
                        .map(Message::Updates)
                }),
                Some(self.workspaces.subscription().map(Message::Workspaces)),
                Some(self.window_title.subscription().map(Message::Title)),
                Some(self.system_info.subscription().map(Message::SystemInfo)),
                Some(self.clock.subscription().map(Message::Clock)),
                Some(self.privacy.subscription().map(Message::Privacy)),
                Some(self.settings.subscription().map(Message::Settings)),
                Some(config::subscription()),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>(),
        )
    }
}

use crate::config::Appearance;
use iced::{
    theme::{self, palette, Palette},
    widget::{button, container, slider},
    Border, Color, Theme,
};

pub fn ashell_theme(appearance: &Appearance) -> Theme {
    Theme::custom_with_fn(
        "local".to_string(),
        Palette {
            background: appearance.background_color.get_base(),
            text: appearance.text_color.get_base(),
            primary: appearance.primary_color.get_base(),
            success: appearance.success_color.get_base(),
            danger: appearance.danger_color.get_base(),
        },
        |palette| {
            let default_bg = palette::Background::new(
                palette.background,
                appearance
                    .background_color
                    .get_text()
                    .unwrap_or(palette.text),
            );
            let default_primary = palette::Primary::generate(
                palette.primary,
                palette.background,
                appearance.primary_color.get_text().unwrap_or(palette.text),
            );
            let default_secondary = palette::Primary::generate(
                appearance.secondary_color.get_base(),
                palette.background,
                appearance
                    .secondary_color
                    .get_text()
                    .unwrap_or(palette.text),
            );
            let default_success = palette::Success::generate(
                palette.success,
                palette.background,
                appearance.success_color.get_text().unwrap_or(palette.text),
            );
            let default_danger = palette::Danger::generate(
                palette.danger,
                palette.background,
                appearance.danger_color.get_text().unwrap_or(palette.text),
            );

            palette::Extended {
                background: palette::Background {
                    base: default_bg.base,
                    weak: appearance
                        .background_color
                        .get_weak_pair(palette.text)
                        .unwrap_or(default_bg.weak),
                    strong: appearance
                        .background_color
                        .get_strong_pair(palette.text)
                        .unwrap_or(default_bg.strong),
                },
                primary: palette::Primary {
                    base: default_primary.base,
                    weak: appearance
                        .primary_color
                        .get_weak_pair(palette.text)
                        .unwrap_or(default_primary.weak),
                    strong: appearance
                        .primary_color
                        .get_strong_pair(palette.text)
                        .unwrap_or(default_primary.strong),
                },
                secondary: palette::Secondary {
                    base: default_secondary.base,
                    weak: appearance
                        .secondary_color
                        .get_weak_pair(palette.text)
                        .unwrap_or(default_secondary.weak),
                    strong: appearance
                        .secondary_color
                        .get_strong_pair(palette.text)
                        .unwrap_or(default_secondary.strong),
                },
                success: palette::Success {
                    base: default_success.base,
                    weak: appearance
                        .success_color
                        .get_weak_pair(palette.text)
                        .unwrap_or(default_success.weak),
                    strong: appearance
                        .success_color
                        .get_strong_pair(palette.text)
                        .unwrap_or(default_success.strong),
                },
                danger: palette::Danger {
                    base: default_danger.base,
                    weak: appearance
                        .danger_color
                        .get_weak_pair(palette.text)
                        .unwrap_or(default_danger.weak),
                    strong: appearance
                        .danger_color
                        .get_strong_pair(palette.text)
                        .unwrap_or(default_danger.strong),
                },
                is_dark: true,
            }
        },
    )
}

pub fn header_pills(theme: &Theme) -> container::Appearance {
    let palette = theme.palette();
    container::Appearance {
        background: Some(palette.background.into()),
        border: Border {
            width: 0.0,
            radius: 12.0.into(),
            color: Color::TRANSPARENT,
        },
        text_color: Some(palette.text),
        ..Default::default()
    }
}

pub fn left_header_pills(theme: &Theme) -> container::Appearance {
    let palette = theme.palette();
    container::Appearance {
        background: Some(palette.background.into()),
        border: Border {
            width: 0.0,
            radius: [12.0, 0.0, 0.0, 12.0].into(),
            color: Color::TRANSPARENT,
        },
        text_color: Some(palette.text),
        ..Default::default()
    }
}

pub enum HeaderButtonStyle {
    Full,
    None,
    Left,
    Right,
}

impl button::StyleSheet for HeaderButtonStyle {
    type Style = theme::Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(style.palette().background.into()),
            border: Border {
                width: 0.0,
                radius: match self {
                    HeaderButtonStyle::Full => 12.0.into(),
                    HeaderButtonStyle::Left => [12.0, 0.0, 0.0, 12.0].into(),
                    HeaderButtonStyle::Right => [0.0, 12.0, 12.0, 0.0].into(),
                    HeaderButtonStyle::None => 0.0.into(),
                },
                color: Color::TRANSPARENT,
            },
            text_color: style.palette().text,
            ..button::Appearance::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(style.extended_palette().background.weak.color.into()),
            ..self.active(style)
        }
    }
}

pub struct GhostButtonStyle;

impl button::StyleSheet for GhostButtonStyle {
    type Style = theme::Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: None,
            border: Border {
                width: 0.0,
                radius: 4.0.into(),
                color: Color::TRANSPARENT,
            },
            text_color: style.palette().text,
            ..button::Appearance::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(style.extended_palette().background.weak.color.into()),
            ..self.active(style)
        }
    }
}

pub struct SettingsButtonStyle;

impl button::StyleSheet for SettingsButtonStyle {
    type Style = theme::Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(style.extended_palette().background.weak.color.into()),
            border: Border {
                width: 0.0,
                radius: 32.0.into(),
                color: Color::TRANSPARENT,
            },
            text_color: style.palette().text,
            ..button::Appearance::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(style.extended_palette().background.strong.color.into()),
            ..self.active(style)
        }
    }
}

pub struct SliderStyle;

impl slider::StyleSheet for SliderStyle {
    type Style = iced::theme::Theme;

    fn active(&self, style: &Self::Style) -> iced::widget::slider::Appearance {
        let palette = style.extended_palette();

        slider::Appearance {
            rail: slider::Rail {
                colors: (palette.primary.base.color, palette.secondary.base.color),
                width: 4.0,
                border_radius: 2.0.into(),
            },
            handle: slider::Handle {
                shape: iced::widget::slider::HandleShape::Circle { radius: 8. },
                color: palette.primary.base.color,
                border_color: palette.primary.base.color,
                border_width: 0.0,
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> iced::widget::slider::Appearance {
        self.active(style)
    }

    fn dragging(&self, style: &Self::Style) -> iced::widget::slider::Appearance {
        self.active(style)
    }
}

pub struct QuickSettingsButtonStyle(pub bool);

impl button::StyleSheet for QuickSettingsButtonStyle {
    type Style = theme::Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(if self.0 {
                style.palette().primary.into()
            } else {
                style.extended_palette().background.weak.color.into()
            }),
            border: Border {
                width: 0.0,
                radius: 32.0.into(),
                color: Color::TRANSPARENT,
            },
            text_color: if self.0 {
                style.extended_palette().primary.base.text
            } else {
                style.palette().text
            },
            ..button::Appearance::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let peach = style.extended_palette().primary.weak.color;

        button::Appearance {
            background: Some(
                if self.0 {
                    peach
                } else {
                    style.extended_palette().background.strong.color
                }
                .into(),
            ),
            ..self.active(style)
        }
    }
}

pub struct QuickSettingsSubMenuButtonStyle(pub bool);

impl button::StyleSheet for QuickSettingsSubMenuButtonStyle {
    type Style = theme::Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: None,
            border: Border {
                width: 0.0,
                radius: 16.0.into(),
                color: Color::TRANSPARENT,
            },
            text_color: if self.0 {
                style.extended_palette().primary.base.text
            } else {
                style.palette().text
            },
            ..button::Appearance::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(style.extended_palette().background.weak.color.into()),
            text_color: style.palette().text,
            ..self.active(style)
        }
    }
}

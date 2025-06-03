use components::buttons::EButton;
use freya::prelude::*;

use crate::components::buttons::Variants;

mod components;

fn main() {
    launch_with_props(app, "Component Library", (500.0, 500.0))
}

fn get_theme(preferred_theme: PreferredTheme) -> Theme {
    match preferred_theme {
        PreferredTheme::Dark => DARK_THEME,
        PreferredTheme::Light => LIGHT_THEME,
    }
}

static PRE_FORK: &[u8] = include_bytes!("./assets/icons/fork.svg");
static PRE_ANCHOR: &[u8] = include_bytes!("./assets/icons/anchor.svg");

fn app() -> Element {
    let preferred_theme = use_preferred_theme();
    let mut current_theme = use_init_theme(|| get_theme(*preferred_theme.peek()));

    use_effect(move || {
        let theme = get_theme(preferred_theme());
        if theme != *current_theme.peek() {
            current_theme.set(theme)
        }
    });

    let fork_icon = static_bytes(PRE_FORK);
    let anchor_icon = static_bytes(PRE_ANCHOR);

    rsx!(
        Body {
            rect {
                width: "fill",
                height: "fill",
                main_align: "center",
                cross_align: "center",
                direction: "vertical",
                rect {
                    direction: "horizontal",
                    rect { margin: "10",
                        EButton {
                            text: "Default",
                        }
                    }
                    rect { margin: "10",
                        EButton {
                            text: "Secondary",
                            variant: Variants::Secondary,
                            svg {
                                svg_data: fork_icon,
                                width: "20",
                                height: "20"
                            }
                        }
                    }
                    rect { margin: "10",
                        EButton {
                            text: "Desctructive",
                            variant: Variants::Destructive,
                        }
                    }
                }
                rect{
                    direction: "horizontal",
                    rect { margin: "10",
                        EButton {
                            text: "Outline",
                            variant: Variants::Outline,
                        }
                    }
                    rect { margin: "10",
                        EButton {
                            text: "Ghost",
                            variant: Variants::Ghost,
                        }
                    }
                    rect { margin: "10",
                        EButton {
                            variant: Variants::Destructive,
                            svg {
                                svg_data: anchor_icon,
                                width: "20",
                                height: "20"
                            }
                        }
                    }
                }
                rect{
                    direction: "horizontal",
                    rect { margin: "10",
                        EButton {
                            theme: theme_with!(ButtonTheme {
                                font_theme: theme_with!(FontTheme {
                                    color: "#FFFFFF".into()
                                }),
                                background: "#123456".into(),
                                border_fill: "#00000000".into(),
                                focus_border_fill: "#654321".into(),
                                hover_background: "#567890".into(),
                                padding: "10".into(),
                            }),
                            text: "Custom theme"
                        }
                    }
                }
            }
        }
    )
}

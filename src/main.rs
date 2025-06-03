use components::normal::CButton;
use freya::prelude::*;
use utils::variants::Variants;

mod components;
mod utils;

fn main() {
    launch_with_props(app, "Component Library", (500.0, 500.0))
}

fn get_theme(preferred_theme: PreferredTheme) -> Theme {
    match preferred_theme {
        PreferredTheme::Dark => DARK_THEME,
        PreferredTheme::Light => LIGHT_THEME,
    }
}

static PRE_ICON: &[u8] = include_bytes!("./assets/icons/fork.svg");

fn app() -> Element {
    let preferred_theme = use_preferred_theme();
    let mut current_theme = use_init_theme(|| get_theme(*preferred_theme.peek()));

    use_effect(move || {
        let theme = get_theme(preferred_theme());
        if theme != *current_theme.peek() {
            current_theme.set(theme)
        }
    });

    let ICON = static_bytes(PRE_ICON);

    rsx!(
        Body {
            rect {
                width: "fill",
                height: "fill",
                main_align: "center",
                cross_align: "center",
                direction: "horizontal",
                rect {
                    CButton {
                        text: "Default",
                    }
                    CButton {
                        text: "Secondary",
                        variant: Variants::Secondary,
                        svg {
                            svg_data: ICON,
                            width: "20",
                            height: "20"
                        }
                    }
                    CButton {
                        text: "Desctructive",
                        variant: Variants::Destructive,
                    }
                }
            }
        }
    )
}

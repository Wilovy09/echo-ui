use freya::prelude::*;

use crate::utils::variants::Variants;

#[allow(non_snake_case)]
#[component]
pub fn COutlineButton(
    text: String,
    click: Option<EventHandler<()>>,
    variant: Option<Variants>,
) -> Element {
    match variant {
        None | Some(Variants::Default) => {
            rsx!(
                OutlineButton{
                  theme: theme_with!(ButtonTheme {
                    font_theme: theme_with!(FontTheme {
                      color: "#000000".into()
                    }),
                    background: "#e5e5e5".into(),
                    border_fill: "#e5e5e500".into(),
                    focus_border_fill: "#cfcfcf00".into(),
                    hover_background: "#cfcfcf".into(),
                    padding: "10".into(),
                  }),
                  onclick: click,
                  label {"{text}"}
                }
            )
        }
        Some(Variants::Secondary) => {
            rsx!(
                OutlineButton{
                  theme: theme_with!(ButtonTheme {
                    font_theme: theme_with!(FontTheme {
                      color: "#FFFFFF".into()
                    }),
                    background: "#262626".into(),
                    border_fill: "#26262600".into(),
                    hover_background: "#202020".into(),
                    focus_border_fill: "#20202000".into(),
                    padding: "10".into(),
                  }),
                  onclick: click,
                  label {"{text}"}
                }
            )
        }
        Some(Variants::Destructive) => {
            rsx!(
                OutlineButton{
                  theme: theme_with!(ButtonTheme {
                    font_theme: theme_with!(FontTheme {
                      color: "#ff0000".into()
                    }),
                    background: "#ffe5e5".into(),
                    border_fill: "#ffe5e500".into(),
                    focus_border_fill: "#ffcfcf00".into(),
                    hover_background: "#ffcfcf".into(),
                    padding: "10".into(),
                  }),
                  onclick: click,
                  label {"{text}"}
                }
            )
        }
    }
}

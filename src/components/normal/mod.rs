use freya::prelude::*;

use crate::utils::variants::Variants;

#[allow(non_snake_case)]
#[component]
pub fn CButton(
    text: String,
    click: Option<EventHandler<()>>,
    variant: Option<Variants>,
    children: Option<Element>,
) -> Element {
    match variant {
        None | Some(Variants::Default) => {
            rsx!(
                Button{
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
                    rect{
                        main_align: "center",
                        cross_align: "center",
                        direction: "horizontal",
                        {children.as_ref()}
                        label { "{text}" }
                    }
                }
            )
        }
        Some(Variants::Secondary) => {
            rsx!(
                Button{
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
                    rect{
                        main_align: "center",
                        cross_align: "center",
                        direction: "horizontal",
                        {children.as_ref()}
                        label { "{text}" }
                    }
                }
            )
        }
        Some(Variants::Destructive) => {
            rsx!(
                Button{
                  theme: theme_with!(ButtonTheme {
                      font_theme: theme_with!(FontTheme {
                          color: "#FFFFFF".into()
                      }),
                      background: "#9e4042".into(),
                      border_fill: "#9e404200".into(),
                      focus_border_fill: "#9e404200".into(),
                      hover_background: "#793132".into(),
                      padding: "10".into(),
                  }),
                  onclick: click,
                    rect{
                        main_align: "center",
                        cross_align: "center",
                        direction: "horizontal",
                        {children.as_ref()}
                        label { "{text}" }
                    }
                }
            )
        }
    }
}

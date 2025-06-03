use freya::prelude::*;

#[derive(Clone, PartialEq)]
pub enum Variants {
    Default,
    Secondary,
    Destructive,
    Outline,
    Ghost,
}

fn get_theme_for_variant(variant: &Variants) -> ButtonThemeWith {
    match variant {
        Variants::Default => theme_with!(ButtonTheme {
            font_theme: theme_with!(FontTheme {
                color: "#000000".into()
            }),
            background: "#e5e5e5".into(),
            border_fill: "#e5e5e500".into(),
            focus_border_fill: "#cfcfcf00".into(),
            hover_background: "#cfcfcf".into(),
            padding: "10".into(),
        }),
        Variants::Secondary => theme_with!(ButtonTheme {
            font_theme: theme_with!(FontTheme {
                color: "#FFFFFF".into()
            }),
            background: "#262626".into(),
            border_fill: "#26262600".into(),
            focus_border_fill: "#20202000".into(),
            hover_background: "#202020".into(),
            padding: "10".into(),
        }),
        Variants::Destructive => theme_with!(ButtonTheme {
            font_theme: theme_with!(FontTheme {
                color: "#FFFFFF".into()
            }),
            background: "#9e4042".into(),
            border_fill: "#9e404200".into(),
            focus_border_fill: "#9e404200".into(),
            hover_background: "#793132".into(),
            padding: "10".into(),
        }),
        Variants::Outline => theme_with!(ButtonTheme {
            font_theme: theme_with!(FontTheme {
                color: "#FFFFFF".into()
            }),
            background: "#151515".into(),
            border_fill: "#373737".into(),
            focus_border_fill: "#3e3e3e".into(),
            hover_background: "#1c1c1c".into(),
            padding: "10".into(),
        }),
        Variants::Ghost => theme_with!(ButtonTheme {
            font_theme: theme_with!(FontTheme {
                color: "#FFFFFF".into()
            }),
            background: "#00000000".into(),
            border_fill: "#00000000".into(),
            focus_border_fill: "#FFFFFF3C".into(),
            hover_background: "#FFFFFF3C".into(),
            padding: "10".into(),
        }),
    }
}

#[allow(non_snake_case)]
#[component]
pub fn EButton(
    text: Option<String>,
    click: Option<EventHandler<()>>,
    variant: Option<Variants>,
    children: Option<Element>,
    theme: Option<ButtonThemeWith>,
) -> Element {
    let effective_theme = theme.unwrap_or_else(|| {
        let variant = variant.unwrap_or(Variants::Default);
        get_theme_for_variant(&variant)
    });

    let label_element = text.as_ref().map(|text| rsx!(label { "{text}" }));

    rsx!(
        Button {
            theme: effective_theme,
            onclick: click,
            rect {
                main_align: "center",
                cross_align: "center",
                direction: "horizontal",
                {children.as_ref()}
                {label_element}
            }
        }
    )
}

#[allow(dead_code)]
#[derive(Clone, PartialEq)]

/*
pub struct ButtonTheme {
    pub font_theme: FontTheme,
    pub background: Cow<'static, str>,
    pub hover_background: Cow<'static, str>,
    pub border_fill: Cow<'static, str>,
    pub focus_border_fill: Cow<'static, str>,
    pub shadow: Cow<'static, str>,
    pub margin: Cow<'static, str>,
    pub corner_radius: Cow<'static, str>,
    pub width: Cow<'static, str>,
    pub height: Cow<'static, str>,
    pub padding: Cow<'static, str>,
}
*/

pub enum Variants {
    Default,
    Secondary,
    Destructive,
}

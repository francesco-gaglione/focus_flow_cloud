use dioxus::prelude::*;
use std::fmt;

#[derive(Copy, Clone, PartialEq)]
pub struct ViewBox {
    pub min_x: u16,
    pub min_y: u16,
    pub width: u16,
    pub height: u16,
}

impl ViewBox {
    pub fn new(min_x: u16, min_y: u16, width: u16, height: u16) -> Self {
        Self {
            min_x,
            min_y,
            width,
            height,
        }
    }
}

impl fmt::Display for ViewBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.min_x, self.min_y, self.width, self.height
        )
    }
}

#[derive(Copy, Clone, PartialEq, Default)]
#[allow(dead_code)]
pub enum LineCap {
    Butt,
    #[default]
    Round,
    Square,
}

impl LineCap {
    fn to_class(self) -> &'static str {
        match self {
            LineCap::Butt => "butt",
            LineCap::Round => "round",
            LineCap::Square => "square",
        }
    }
}

#[derive(Copy, Clone, PartialEq, Default)]
#[allow(dead_code)]
pub enum LineJoin {
    Arcs,
    Bevel,
    Miter,
    MiterClip,
    #[default]
    Round,
}

impl LineJoin {
    fn to_class(self) -> &'static str {
        match self {
            LineJoin::Arcs => "arcs",
            LineJoin::Bevel => "bevel",
            LineJoin::Miter => "miter",
            LineJoin::MiterClip => "miter-clip",
            LineJoin::Round => "round",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct IconProps {
    #[props(default = ViewBox::new(0, 0, 24, 24))]
    pub view_box: ViewBox,
    #[props(default = 2.)]
    pub stroke_width: f64,
    #[props(default)]
    pub stroke_line_cap: LineCap,
    #[props(default)]
    pub stroke_line_join: LineJoin,
    #[props(default = "currentColor")]
    pub stroke: &'static str,
    #[props(default = "none")]
    pub fill: &'static str,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    pub children: Element,
}

#[component]
pub fn Icon(props: IconProps) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: props.view_box.to_string(),
            fill: props.fill,
            stroke: props.stroke,
            stroke_linecap: props.stroke_line_cap.to_class(),
            stroke_linejoin: props.stroke_line_join.to_class(),
            stroke_width: props.stroke_width,
            ..props.attributes,
            {props.children}
        }
    }
}

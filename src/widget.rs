use glib::object::Cast;
use gtk;
use gtk::prelude::*;
use serde::Deserialize;

pub trait Buildable {
    fn build(&self) -> gtk::Widget;
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WidgetDefinition {
    Box(Box),
    Grid(Grid),
    Label(Label),
    Button(Button),
}

impl Buildable for WidgetDefinition {
    fn build(&self) -> gtk::Widget {
        match self {
            WidgetDefinition::Box(b) => b.build(),
            WidgetDefinition::Grid(grid) => grid.build(),
            WidgetDefinition::Label(label) => label.build(),
            WidgetDefinition::Button(button) => button.build(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Box {
    #[serde(default)]
    pub orientation: super::expression::Expr,

    #[serde(default)]
    pub spacing: super::expression::Expr,

    #[serde(rename = "$value", default)]
    pub children: Vec<WidgetDefinition>,
}

impl Default for Box {
    fn default() -> Self {
        Box {
            orientation: "vertical".to_string(),
            spacing: 0,
            children: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Grid {
    #[serde(default)]
    pub orientation: super::expression::Expr,

    #[serde(rename = "$value", default)]
    pub children: Vec<WidgetDefinition>,
}

impl Default for Grid {
    fn default() -> Self {
        Grid {
            orientation: "vertical".to_string(),
            children: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Label {
    #[serde(rename = "$text")]
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct Button {
    #[serde(rename = "$text")]
    pub content: String,
}

impl Default for Button {
    fn default() -> Self {
        Button {
            content: String::new(),
        }
    }
}

impl Buildable for Box {
    fn build(&self) -> gtk::Widget {
        let box_container = gtk::Box::new(gtk::Orientation::Vertical, self.spacing);

        for child in self.children.iter() {
            let child = child.build();
            println!("{:?}", child);
            box_container.append(&child);
        }

        box_container.upcast()
    }
}

impl Buildable for Grid {
    fn build(&self) -> gtk::Widget {
        gtk::Grid::new().upcast()
    }
}

impl Buildable for Label {
    fn build(&self) -> gtk::Widget {
        gtk::Label::new(Some(self.content.as_str())).upcast()
    }
}

impl Buildable for Button {
    fn build(&self) -> gtk::Widget {
        gtk::Button::new().upcast()
    }
}

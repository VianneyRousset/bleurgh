use serde::Deserialize;
use gtk;
use gtk::prelude::*;

use super::widget::Buildable;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")] 
pub struct Window {

    #[serde(rename = "@name", default)]
    pub name: String,

    #[serde(rename = "$value", default)]
    pub widgets: Vec<super::widget::WidgetDefinition>,
}

impl Window {

    pub fn build(&self, app: &gtk::Application) -> gtk::ApplicationWindow {

        let widgets = self.widgets.iter().map(|w| w.build());

        let window = gtk::ApplicationWindow::new(app);
        let box_container = gtk::Box::new(gtk::Orientation::Vertical, 5);

        for widget in widgets {
            println!("{:?}", widget);
            box_container.append(&widget);
        }

        window.set_child(Some(&box_container));
        window.set_title(Some(self.name.as_str()));
        window
    }

}


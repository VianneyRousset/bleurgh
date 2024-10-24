//use gtk;

// brings in many useful traits that extend the functionality of GTK objects
//use gtk::prelude::*;

mod engine;
//mod widget;

use self::engine::expression::Expr;
use self::engine::{Engine, EngineError, StoreVariable};
use boa_engine;

fn main() -> Result<(), EngineError> {
    Ok(())

    //app.connect_activate(build_ui);
    //app.run();
}

/*
fn build_ui(app: &gtk::Application) {
    let conf = config::load_config_file("test.xml").expect("Failed to load config file");
    println!("{:#?}", conf);

    let windows: Vec<gtk::ApplicationWindow> = conf
        .windows
        .iter()
        .map(|w: &config::window::Window| w.build(app))
        .collect();

    windows.iter().for_each(|w| w.present());
}
*/

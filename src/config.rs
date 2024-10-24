use serde::Deserialize;
use serde_path_to_error::deserialize;

pub mod template;
pub mod widget;
pub mod window;

#[derive(Debug, Deserialize)]
pub struct Config {
    //#[serde(rename = "variable", default)]
    //variables: Vec<variable:VariableDefinition>,
    #[serde(rename = "window", default)]
    pub windows: Vec<window::Window>,
}

pub fn load_config_file(filepath: &str) -> Result<Config, String> {
    //let f = File::open(filename).expect(&format!("Cannot open file {}", filename));
    //let r = BufReader::new(f);
    let xml = std::fs::read_to_string(filepath).unwrap();
    let mut deserializer = quick_xml::de::Deserializer::from_str(&xml);

    match deserialize::<_, Config>(&mut deserializer) {
        Ok(result) => return Ok(result),
        Err(err) => return Err(format!("Failed to load config file: {}", err)),
    };
}

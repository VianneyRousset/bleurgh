use boa_engine::property::PropertyDescriptorBuilder;
use boa_engine::{JsString, JsValue};

use super::error::EngineError;
use super::{Engine, Json};

pub trait StoreVariable<T> {
    #[must_use]
    fn set_variable(&mut self, key: &str, value: T) -> Result<(), EngineError>;
}
impl StoreVariable<JsValue> for Engine {
    fn set_variable(&mut self, key: &str, value: JsValue) -> Result<(), EngineError> {
        let key = JsString::from(key);
        let prop = PropertyDescriptorBuilder::new()
            .value(value.clone())
            .writable(false)
            .enumerable(true)
            .configurable(true)
            .build();

        let global_object = self.context.global_object();
        match global_object.define_property_or_throw(key, prop, &mut self.context) {
            Ok(_) => Ok(()),
            Err(error) => Err(EngineError::from(error)),
        }
    }
}

impl StoreVariable<Json> for Engine {
    fn set_variable(&mut self, key: &str, value: Json) -> Result<(), EngineError> {
        let value = JsValue::from_json(&value, &mut self.context)?;
        self.set_variable(key, value)
    }
}

impl StoreVariable<char> for Engine {
    fn set_variable(&mut self, key: &str, value: char) -> Result<(), EngineError> {
        let value: JsValue = value.into();
        self.set_variable(key, value)
    }
}

impl StoreVariable<bool> for Engine {
    fn set_variable(&mut self, key: &str, value: bool) -> Result<(), EngineError> {
        let value: JsValue = value.into();
        self.set_variable(key, value)
    }
}

impl StoreVariable<u8> for Engine {
    fn set_variable(&mut self, key: &str, value: u8) -> Result<(), EngineError> {
        let value: JsValue = value.into();
        self.set_variable(key, value)
    }
}

impl StoreVariable<i8> for Engine {
    fn set_variable(&mut self, key: &str, value: i8) -> Result<(), EngineError> {
        let value: JsValue = value.into();
        self.set_variable(key, value)
    }
}

impl StoreVariable<u16> for Engine {
    fn set_variable(&mut self, key: &str, value: u16) -> Result<(), EngineError> {
        let value: JsValue = value.into();
        self.set_variable(key, value)
    }
}

impl StoreVariable<i16> for Engine {
    fn set_variable(&mut self, key: &str, value: i16) -> Result<(), EngineError> {
        let value: JsValue = value.into();
        self.set_variable(key, value)
    }
}

impl StoreVariable<u32> for Engine {
    fn set_variable(&mut self, key: &str, value: u32) -> Result<(), EngineError> {
        let value: JsValue = value.into();
        self.set_variable(key, value)
    }
}

impl StoreVariable<i32> for Engine {
    fn set_variable(&mut self, key: &str, value: i32) -> Result<(), EngineError> {
        let value: JsValue = value.into();
        self.set_variable(key, value)
    }
}

impl StoreVariable<u64> for Engine {
    fn set_variable(&mut self, key: &str, value: u64) -> Result<(), EngineError> {
        let value: JsValue = value.into();
        self.set_variable(key, value)
    }
}

impl StoreVariable<i64> for Engine {
    fn set_variable(&mut self, key: &str, value: i64) -> Result<(), EngineError> {
        let value: JsValue = value.into();
        self.set_variable(key, value)
    }
}

impl StoreVariable<f32> for Engine {
    fn set_variable(&mut self, key: &str, value: f32) -> Result<(), EngineError> {
        let value: JsValue = value.into();
        self.set_variable(key, value)
    }
}

impl StoreVariable<f64> for Engine {
    fn set_variable(&mut self, key: &str, value: f64) -> Result<(), EngineError> {
        let value: JsValue = value.into();
        self.set_variable(key, value)
    }
}

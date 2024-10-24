use boa_engine::{Context, Script, Source};
use serde_json;

pub mod error;
pub mod expression;
pub mod variable;

pub use self::error::EngineError;
pub use self::expression::{Expr, ParsedExpr};
pub use self::variable::StoreVariable;

type Json = serde_json::Value;

#[derive(Debug)]
pub struct Engine {
    context: Context,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            context: Context::default(),
        }
    }

    pub fn parse(&mut self, expr: &Expr) -> Result<ParsedExpr, EngineError> {
        match Script::parse(Source::from_bytes(&expr.js_code), None, &mut self.context) {
            Ok(script) => Ok(ParsedExpr::new(script)),
            Err(error) => return Err(EngineError::from(error)),
        }
    }

    fn eval_bool(&mut self, expr: &ParsedExpr) -> Result<bool, EngineError> {
        match expr.script.evaluate(&mut self.context) {
            Ok(value) => Ok(value.to_boolean()),
            Err(error) => Err(EngineError::JsError(error)),
        }
    }

    fn eval_u8(&mut self, expr: &ParsedExpr) -> Result<u8, EngineError> {
        match expr.script.evaluate(&mut self.context) {
            Ok(value) => value
                .to_uint8(&mut self.context)
                .or_else(|e| Err(EngineError::from(e))),
            Err(error) => Err(EngineError::JsError(error)),
        }
    }

    fn eval_i8(&mut self, expr: &ParsedExpr) -> Result<i8, EngineError> {
        match expr.script.evaluate(&mut self.context) {
            Ok(value) => value
                .to_int8(&mut self.context)
                .or_else(|e| Err(EngineError::from(e))),
            Err(error) => Err(EngineError::JsError(error)),
        }
    }

    fn eval_u16(&mut self, expr: &ParsedExpr) -> Result<u16, EngineError> {
        match expr.script.evaluate(&mut self.context) {
            Ok(value) => value
                .to_uint16(&mut self.context)
                .or_else(|e| Err(EngineError::from(e))),
            Err(error) => Err(EngineError::JsError(error)),
        }
    }

    fn eval_i16(&mut self, expr: &ParsedExpr) -> Result<i16, EngineError> {
        match expr.script.evaluate(&mut self.context) {
            Ok(value) => value
                .to_int16(&mut self.context)
                .or_else(|e| Err(EngineError::from(e))),
            Err(error) => Err(EngineError::JsError(error)),
        }
    }

    fn eval_u32(&mut self, expr: &ParsedExpr) -> Result<u32, EngineError> {
        match expr.script.evaluate(&mut self.context) {
            Ok(value) => value
                .to_u32(&mut self.context)
                .or_else(|e| Err(EngineError::from(e))),
            Err(error) => Err(EngineError::JsError(error)),
        }
    }

    fn eval_i32(&mut self, expr: &ParsedExpr) -> Result<i32, EngineError> {
        match expr.script.evaluate(&mut self.context) {
            Ok(value) => value
                .to_i32(&mut self.context)
                .or_else(|e| Err(EngineError::from(e))),
            Err(error) => Err(EngineError::JsError(error)),
        }
    }

    fn eval_f32(&mut self, expr: &ParsedExpr) -> Result<f32, EngineError> {
        match expr.script.evaluate(&mut self.context) {
            Ok(value) => match value.to_numeric_number(&mut self.context) {
                Ok(value) => Ok(value as f32),
                Err(error) => Err(EngineError::from(error)),
            },
            Err(error) => Err(EngineError::JsError(error)),
        }
    }

    fn eval_f64(&mut self, expr: &ParsedExpr) -> Result<f64, EngineError> {
        match expr.script.evaluate(&mut self.context) {
            Ok(value) => value
                .to_numeric_number(&mut self.context)
                .or_else(|e| Err(EngineError::from(e))),
            Err(error) => Err(EngineError::JsError(error)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::{Engine, Expr};

    #[test]
    fn test_expr_parsing() {
        let mut engine = Engine::new();
        let expr = Expr::new(String::from("42"));

        assert!(engine.parse(&expr).is_ok());
    }

    #[test]
    fn test_expr_parsing_fail() {
        let mut engine = Engine::new();
        let expr = Expr::new(String::from("{;"));

        assert!(engine.parse(&expr).is_err());
    }

    #[test]
    fn test_eval_bool() {
        let mut engine = Engine::new();
        let expr = Expr::new(String::from("false"));
        let parsed_expr = engine.parse(&expr).unwrap();
        assert_eq!(engine.eval_bool(&parsed_expr).unwrap(), false);
    }

    #[test]
    fn test_eval_u8() {
        let mut engine = Engine::new();
        let expr = Expr::new(String::from("42"));
        let parsed_expr = engine.parse(&expr).unwrap();
        assert_eq!(engine.eval_u8(&parsed_expr).unwrap(), 42);
    }

    #[test]
    fn test_eval_i8() {
        let mut engine = Engine::new();
        let expr = Expr::new(String::from("42"));
        let parsed_expr = engine.parse(&expr).unwrap();
        assert_eq!(engine.eval_i8(&parsed_expr).unwrap(), 42);
    }

    #[test]
    fn test_eval_u16() {
        let mut engine = Engine::new();
        let expr = Expr::new(String::from("42"));
        let parsed_expr = engine.parse(&expr).unwrap();
        assert_eq!(engine.eval_u16(&parsed_expr).unwrap(), 42);
    }

    #[test]
    fn test_eval_i16() {
        let mut engine = Engine::new();
        let expr = Expr::new(String::from("42"));
        let parsed_expr = engine.parse(&expr).unwrap();
        assert_eq!(engine.eval_i16(&parsed_expr).unwrap(), 42);
    }

    #[test]
    fn test_eval_u32() {
        let mut engine = Engine::new();
        let expr = Expr::new(String::from("42"));
        let parsed_expr = engine.parse(&expr).unwrap();
        assert_eq!(engine.eval_u32(&parsed_expr).unwrap(), 42);
    }

    #[test]
    fn test_eval_i32() {
        let mut engine = Engine::new();
        let expr = Expr::new(String::from("42"));
        let parsed_expr = engine.parse(&expr).unwrap();
        assert_eq!(engine.eval_i32(&parsed_expr).unwrap(), 42);
    }

    #[test]
    fn test_eval_f32() {
        let mut engine = Engine::new();
        let expr = Expr::new(String::from("7.5"));
        let parsed_expr = engine.parse(&expr).unwrap();
        assert_eq!(engine.eval_f32(&parsed_expr).unwrap(), 7.5);
    }

    #[test]
    fn test_eval_f64() {
        let mut engine = Engine::new();
        let expr = Expr::new(String::from("7.5"));
        let parsed_expr = engine.parse(&expr).unwrap();
        assert_eq!(engine.eval_f64(&parsed_expr).unwrap(), 7.5);
    }
}

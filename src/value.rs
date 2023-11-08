
use crate::object::Object;

pub enum Value {
    Bool(bool),
    Integer(i32),
    Float(f64),
    Complex(f64),
    Object(Object)
}
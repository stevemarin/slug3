
use std::ops::{Add, Sub, Mul, Div};

use num::complex::Complex;

use crate::object::ObjFucntion;


#[derive(Debug, Clone, Copy)]
pub enum Value<'vm> {
    Bool(bool),
    Integer(i32),
    Float(f64),
    Complex(Complex<f64>),
    Object(&'vm ObjFucntion<'vm>)
}

impl<'a> Add<Value<'a>> for Value<'a> {
    type Output = Value<'a>;
    fn add(self, rhs: Value) -> Self {
        match (self, rhs) {
            (Value::Integer(a), Value::Integer(b)) => Value::Integer(a + b),
            // (Value::Integer(a), Value::Float(b)) => Value::Float(a as f64 + b),
            // (Value::Integer(a), Value::Complex(b)) => Value::Complex(Complex::new(a as f64, 0.0) + b),
            // (Value::Float(a), Value::Integer(b)) => Value::Float(a + b as f64),
            (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
            // (Value::Float(a), Value::Complex(b)) => Value::Complex(Complex::new(a, 0.0) + b),
            // (Value::Complex(a), Value::Integer(b)) => Value::Complex(a + Complex::new(b as f64, 0.0)),
            // (Value::Complex(a), Value::Float(b)) => Value::Complex(a + Complex::new(b, 0.0)),
            (Value::Complex(a), Value::Complex(b)) => Value::Complex(a + b),
            _ => unreachable!()
        }
    }
}

impl<'a> Sub<Value<'a>> for Value<'a> {
    type Output = Value<'a>;
    fn sub(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::Integer(a), Value::Integer(b)) => Value::Integer(a - b),
            // (Value::Integer(a), Value::Float(b)) => Value::Float(a as f64 - b),
            // (Value::Integer(a), Value::Complex(b)) => Value::Complex(Complex::new(a as f64, 0.0) - b),
            // (Value::Float(a), Value::Integer(b)) => Value::Float(a - b as f64),
            (Value::Float(a), Value::Float(b)) => Value::Float(a - b),
            // (Value::Float(a), Value::Complex(b)) => Value::Complex(Complex::new(a, 0.0) - b),
            // (Value::Complex(a), Value::Integer(b)) => Value::Complex(a - Complex::new(b as f64, 0.0)),
            // (Value::Complex(a), Value::Float(b)) => Value::Complex(a - Complex::new(b, 0.0)),
            (Value::Complex(a), Value::Complex(b)) => Value::Complex(a - b),
            _ => unreachable!()
        }
    }
}

impl<'a> Mul<Value<'a>> for Value<'a> {
    type Output = Value<'a>;
    fn mul(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::Integer(a), Value::Integer(b)) => Value::Integer(a * b),
            // (Value::Integer(a), Value::Float(b)) => Value::Float(a as f64 * b),
            // (Value::Integer(a), Value::Complex(b)) => Value::Complex(Complex::new(a as f64, 0.0) * b),
            // (Value::Float(a), Value::Integer(b)) => Value::Float(a * b as f64),
            (Value::Float(a), Value::Float(b)) => Value::Float(a * b),
            // (Value::Float(a), Value::Complex(b)) => Value::Complex(Complex::new(a, 0.0) * b),
            // (Value::Complex(a), Value::Integer(b)) => Value::Complex(a * Complex::new(b as f64, 0.0)),
            // (Value::Complex(a), Value::Float(b)) => Value::Complex(a * Complex::new(b, 0.0)),
            (Value::Complex(a), Value::Complex(b)) => Value::Complex(a * b),
            _ => unreachable!()
        }
    }
}

impl<'a> Div<Value<'a>> for Value<'a> {
    type Output = Value<'a>;
    fn div(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::Integer(a), Value::Integer(b)) => Value::Float((a / b) as f64),
            // (Value::Integer(a), Value::Float(b)) => Value::Float(a as f64 / b),
            // (Value::Integer(a), Value::Complex(b)) => Value::Complex(Complex::new(a as f64, 0.0) / b),
            // (Value::Float(a), Value::Integer(b)) => Value::Float(a / b as f64),
            (Value::Float(a), Value::Float(b)) => Value::Float(a / b),
            // (Value::Float(a), Value::Complex(b)) => Value::Complex(Complex::new(a, 0.0) / b),
            // (Value::Complex(a), Value::Integer(b)) => Value::Complex(a / Complex::new(b as f64, 0.0)),
            // (Value::Complex(a), Value::Float(b)) => Value::Complex(a / Complex::new(b, 0.0)),
            (Value::Complex(a), Value::Complex(b)) => Value::Complex(a / b),
            _ => unreachable!()
        }
    }
}

// impl<'a> PartialOrd for Value<'a> {
//     fn ge(&self, other: &Self) -> bool {
        
//     }
// }

// impl<'a> PartialEq for Value<'a> {
//     fn eq(&self, other: &Self) -> bool {
        
//     }

//     fn ne(&self, other: &Self) -> bool {
        
//     }
    
//     // type Output = Value<'a>;
//     // fn eq(self, rhs: Value) -> Self:: {
//     //     match (self, rhs) {
//     //         (Value::Integer(a), Value::Integer(b)) => Value::Bool(a == b),
//     //         (Value::Integer(a), Value::Float(b)) => Value::Float(a as f64 / b),
//     //         (Value::Integer(a), Value::Complex(b)) => Value::Complex(Complex::new(a as f64, 0.0) / b),
//     //         (Value::Float(a), Value::Integer(b)) => Value::Float(a / b as f64),
//     //         (Value::Float(a), Value::Float(b)) => Value::Float(a / b),
//     //         (Value::Float(a), Value::Complex(b)) => Value::Complex(Complex::new(a, 0.0) / b),
//     //         (Value::Complex(a), Value::Integer(b)) => Value::Complex(a / Complex::new(b as f64, 0.0)),
//     //         (Value::Complex(a), Value::Float(b)) => Value::Complex(a / Complex::new(b, 0.0)),
//     //         (Value::Complex(a), Value::Complex(b)) => Value::Complex(a / b),
//     //         _ => unreachable!()
//     //     }
//     // }
// }

impl<'a> Value<'a> {
    pub fn pow(self, rhs: Value) -> Self {
        match (self, rhs) {
            (Value::Integer(a), Value::Integer(b)) => Value::Float((a as f64).powf(b as f64)),
            // (Value::Integer(a), Value::Float(b)) => Value::Float((a as f64).powf(b)),
            // (Value::Integer(a), Value::Complex(b)) => Value::Complex(Complex::new(a as f64, 0.0).powc(b)),
            (Value::Float(a), Value::Integer(b)) => Value::Float(a.powf(b as f64)),
            (Value::Float(a), Value::Float(b)) => Value::Float(a.powf(b)),
            // (Value::Float(a), Value::Complex(b)) => Value::Complex(Complex::new(a, 0.0).powc(b)),
            (Value::Complex(a), Value::Integer(b)) => Value::Complex(a.powc(Complex::new(b as f64, 0.0))),
            // (Value::Complex(a), Value::Float(b)) => Value::Complex(a.powc(Complex::new(b, 0.0))),
            (Value::Complex(a), Value::Complex(b)) => Value::Complex(a.powc(b)),
            _ => unreachable!()
        }
    }

    pub fn int_division(self, rhs: Value) -> Self {
        match (self, rhs) {
            (Value::Integer(a), Value::Integer(b)) => Value::Integer((a / b) as i32),
            (Value::Integer(a), Value::Float(b)) => Value::Float(((a as f64) / b) as i32 as f64),
            (Value::Float(a), Value::Integer(b)) => Value::Float((a / (b as f64)) as i32 as f64),
            (Value::Float(a), Value::Float(b)) => Value::Float((a / b) as i32 as f64),
            _ => unreachable!()
        }
    }

}

#[inline]
pub fn is_number<'vm>(value: &'vm Value<'vm>) -> bool {
    match value {
        Value::Integer(_) | Value::Float(_) | Value::Complex(_) => true,
        _ => false
    }
}

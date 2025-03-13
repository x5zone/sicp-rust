// src/listv.rs:
use std::{any::Any, error::Error, fmt::Debug, io, ptr};

/// A macro to check if a `ListV` object is of a specific type.
/// 判断 `ListV` 是否为某种具体类型的宏。
///
/// # Examples
///
/// ```
/// use sicp_rs::is_type;
/// use sicp_rs::listv::ListV;
///
/// let value: Box<dyn ListV> = Box::new("hello".to_string());
/// assert!(is_type!(value, String));
/// assert!(!is_type!(value, i32));
/// ```
#[macro_export]
macro_rules! is_type {
    ($value:expr, $ty:ty) => {
        $value.as_any().downcast_ref::<$ty>().is_some()
    };
}
/// A trait representing a dynamic type used as the base type in SICP-style lists.
/// 定义一个动态类型，用于 SICP 风格的列表操作，所有类型在插入 List 列表前都将被转换为 `ListV` 类型。
///
/// # Overview
///
/// This trait allows any type to be wrapped and used in a dynamically typed list.
/// It provides methods for type checking, equality, and string conversion.
///
/// 此 trait 提供了类型检查、相等性判断和字符串转换等功能，
/// 支持将任意类型封装为 `ListV` 动态类型。
pub trait ListV: Any + Debug {
    /// Returns a reference to the underlying `Any` type.
    fn as_any(&self) -> &dyn Any;

    /// Checks if the object is a string (`String` or `&str`).
    fn is_string(&self) -> bool;

    /// Checks if the object is an integer type (`i8`, `i16`, ..., `u128`).
    fn is_integer(&self) -> bool;

    /// Checks if the object is a floating-point type (`f32` or `f64`).
    fn is_float(&self) -> bool;

    /// Determines if the object is the same instance as another `ListV` object.
    /// This is based on pointer equality, inspired by the SICP concept of sameness.
    /// 该方法基于指针比较，源于 SICP 中的“同一性”概念。
    fn sameness(&self, other: &dyn ListV) -> bool {
        ptr::eq(self.as_any() as *const _, other.as_any() as *const _)
    }
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}
impl<T> ListV for T
where
    T: Any + Debug + Clone + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn is_string(&self) -> bool {
        is_type!(self, String) || is_type!(self, &str)
    }
    fn is_integer(&self) -> bool {
        is_type!(self, i8)
            || is_type!(self, i16)
            || is_type!(self, i32)
            || is_type!(self, i64)
            || is_type!(self, i128)
            || is_type!(self, u8)
            || is_type!(self, u16)
            || is_type!(self, u32)
            || is_type!(self, u64)
            || is_type!(self, u128)
    }
    fn is_float(&self) -> bool {
        is_type!(self, f32) || is_type!(self, f64)
    }
}

impl PartialEq for dyn ListV {
    fn eq(&self, other: &Self) -> bool {
        // String comparison: Treat `String` and `&str` as equal if their values are the same.
        if self.is_string() && other.is_string() {
            let judge = |a: &dyn ListV, b: &dyn ListV| {
                if let (Some(a), Some(b)) = (
                    a.as_any().downcast_ref::<String>(),
                    b.as_any().downcast_ref::<&str>(),
                ) {
                    a == b
                } else {
                    false
                }
            };
            judge(self, other)
                || judge(other, self)
                || compare_as::<String>(self, other)
                || compare_as::<&str>(self, other)
        } else if self.is_float() && other.is_float() {
            // Float comparison: Promote to `f64` for strict comparison.
            if let (Ok(a), Ok(b)) = (to_f64(self), to_f64(other)) {
                const ABS_EPSILON: f64 = 100.0 * f64::EPSILON;
                let dynamic_epsilon = f64::EPSILON * a.abs().max(b.abs());
                if (a - b).abs() < ABS_EPSILON || (a - b).abs() < dynamic_epsilon {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else if self.is_integer() && other.is_integer() {
            // Integer comparison: Promote to `i128` or `u128` for comparison. eg: 1_i32 == 1_u64
            if let (Ok(a), Ok(b)) = (to_i128(self), to_i128(other)) {
                a == b
            } else if let (Ok(a), Ok(b)) = (to_u128(self), to_u128(other)) {
                a == b
            } else {
                false
            }
        } else if self.type_id() == other.type_id() && self.to_string() == other.to_string() {
            // Non-primitive types: Compare by string representation.
            true
        } else {
            false
        }
    }
}

impl PartialOrd for dyn ListV {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.is_float() && other.is_float() {
            if let (Ok(a), Ok(b)) = (to_f64(self), to_f64(other)) {
                a.partial_cmp(&b)
            } else {
                None
            }
        } else if self.is_integer() && other.is_integer() {
            if let (Ok(a), Ok(b)) = (to_i128(self), to_i128(other)) {
                a.partial_cmp(&b)
            } else if let (Ok(a), Ok(b)) = (to_u128(self), to_u128(other)) {
                a.partial_cmp(&b)
            } else {
                None
            }
        } else if self.is_string() && other.is_string() {
            self.to_string().partial_cmp(&other.to_string())
        } else if self.type_id() == other.type_id() {
            self.to_string().partial_cmp(&other.to_string())
        } else {
            None
        }
    }
}

// Helper functions for type conversion and comparison
fn compare_as<T: PartialEq + 'static>(a: &dyn ListV, b: &dyn ListV) -> bool {
    if let (Some(a), Some(b)) = (
        a.as_any().downcast_ref::<T>(),
        b.as_any().downcast_ref::<T>(),
    ) {
        a == b
    } else {
        false
    }
}
fn to_i128(value: &dyn ListV) -> Result<i128, Box<dyn Error>> {
    if let Some(value) = value.as_any().downcast_ref::<i8>() {
        Ok(*value as i128)
    } else if let Some(value) = value.as_any().downcast_ref::<i16>() {
        Ok(*value as i128)
    } else if let Some(value) = value.as_any().downcast_ref::<i32>() {
        Ok(*value as i128)
    } else if let Some(value) = value.as_any().downcast_ref::<i64>() {
        Ok(*value as i128)
    } else if let Some(value) = value.as_any().downcast_ref::<i128>() {
        Ok(*value)
    } else if let Some(value) = value.as_any().downcast_ref::<u8>() {
        Ok(*value as i128)
    } else if let Some(value) = value.as_any().downcast_ref::<u16>() {
        Ok(*value as i128)
    } else if let Some(value) = value.as_any().downcast_ref::<u32>() {
        Ok(*value as i128)
    } else if let Some(value) = value.as_any().downcast_ref::<u64>() {
        Ok(*value as i128)
    } else if let Some(value) = value.as_any().downcast_ref::<u128>() {
        i128::try_from(*value).map_err(|e| Box::new(e) as Box<dyn Error>)
    } else {
        Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidData,
            "Unsupported type",
        )))
    }
}
fn to_u128(value: &dyn ListV) -> Result<u128, Box<dyn Error>> {
    if let Some(value) = value.as_any().downcast_ref::<i8>() {
        u128::try_from(*value).map_err(|e| Box::new(e) as Box<dyn Error>)
    } else if let Some(value) = value.as_any().downcast_ref::<i16>() {
        u128::try_from(*value).map_err(|e| Box::new(e) as Box<dyn Error>)
    } else if let Some(value) = value.as_any().downcast_ref::<i32>() {
        u128::try_from(*value).map_err(|e| Box::new(e) as Box<dyn Error>)
    } else if let Some(value) = value.as_any().downcast_ref::<i64>() {
        u128::try_from(*value).map_err(|e| Box::new(e) as Box<dyn Error>)
    } else if let Some(value) = value.as_any().downcast_ref::<i128>() {
        u128::try_from(*value).map_err(|e| Box::new(e) as Box<dyn Error>)
    } else if let Some(value) = value.as_any().downcast_ref::<u8>() {
        Ok(*value as u128)
    } else if let Some(value) = value.as_any().downcast_ref::<u16>() {
        Ok(*value as u128)
    } else if let Some(value) = value.as_any().downcast_ref::<u32>() {
        Ok(*value as u128)
    } else if let Some(value) = value.as_any().downcast_ref::<u64>() {
        Ok(*value as u128)
    } else if let Some(value) = value.as_any().downcast_ref::<u128>() {
        Ok(*value)
    } else {
        Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidData,
            "Unsupported type",
        )))
    }
}
fn to_f64(value: &dyn ListV) -> Result<f64, Box<dyn Error>> {
    if let Some(value) = value.as_any().downcast_ref::<f32>() {
        Ok(*value as f64)
    } else if let Some(value) = value.as_any().downcast_ref::<f64>() {
        Ok(*value)
    } else {
        Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidData,
            "Unsupported type",
        )))
    }
}

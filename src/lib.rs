
use itertools::Itertools;
use serde_json::Value;

#[cfg(test)]
pub mod tests;

pub trait NullEq {
    /// Compares two json values for equality. This isn't strict equality, as json can be a bit fuzzy. 
    /// 
    /// - `undefined` == `null` == [`Option::None`] == [`Value::Null`]
    /// - For objects: A missing key is treated as a [`Value::Null`]
    /// - For arrays: All the elements are the same and in the same order
    fn null_eq(&self, other: &Self) -> bool;
}

impl NullEq for Value {
    fn null_eq(&self, other: &Self) -> bool {
        match self {
            Value::Null => other.is_null(),

            Value::Array(val) => other.as_array().is_some_and(|val2| val.null_eq(val2)),
            Value::Object(val) => other.as_object().is_some_and(|val2| val.null_eq(val2)),

            Value::Bool(val) => other.as_bool().is_some_and(|val2| val.eq(&val2)),
            Value::Number(val) => other.as_number().is_some_and(|val2| val.eq(val2)),
            Value::String(val) => other.as_str().is_some_and(|val2| val.eq(val2)),
        }
    }
}

impl NullEq for Vec<Value> {
    fn null_eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        for i in 0..self.len() {
            if !self
                .get(i)
                .is_some_and(|val1| other.get(i).is_some_and(|val2| val1.null_eq(val2)))
            {
                return false;
            }
        }

        true
    }
}

impl NullEq for serde_json::Map<String, Value> {
    fn null_eq(&self, other: &Self) -> bool {
        let key_iter = self.keys().merge(other.keys()).unique();

        for key in key_iter {
            let self_value = self.get(key).unwrap_or(&Value::Null);
            let other_value = other.get(key).unwrap_or(&Value::Null);

            if !self_value.null_eq(other_value) {
                return false;
            }
        }

        true
    }
}
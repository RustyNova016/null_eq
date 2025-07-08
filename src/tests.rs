use serde_json::json;
use serde_json::Value;

use crate::NullEq;

pub fn assert_null_eq(a: &Value, b: &Value) {
    if !a.null_eq(b)  {
        panic!("{a:#?} is not equal to {b:#?}")
    }

    if !b.null_eq(a)  {
        panic!("{b:#?} is not equal to {a:#?}")
    }
}

#[test]
fn empty_object_test() {
    let a = json!({});
    let b = json!({"test": Value::Null});

    assert_null_eq(&a, &b);
}
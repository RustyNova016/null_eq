# Null Eq

A small library to compare json values. This compares 2 `serde_json::Value`s together 
and return true if they are equal, with some fuzzyness:
 - `undefined` == `null` == `Option::None` == `Value::Null`
 - For objects: A missing key is treated as a `Value::Null`
 - For arrays: All the elements are the same and in the same order
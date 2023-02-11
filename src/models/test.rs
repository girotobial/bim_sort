use serde;
use serde_json;
pub(crate) trait FromValue {
    fn from_value(value: &serde_json::Value) -> Self;
}

impl<T: for<'a> serde::Deserialize<'a>> FromValue for T {
    fn from_value(value: &serde_json::Value) -> Self {
        serde_json::from_str(value.to_string().as_str()).expect("Could not convert input.")
    }
}

pub(crate) trait ToValue {
    fn to_value(&self) -> serde_json::Value;
}

impl<T: serde::Serialize> ToValue for T {
    fn to_value(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Could not convert back to value")
    }
}

pub(crate) fn there_and_back_test<T: ToValue, F>(input: serde_json::Value, f: F)
where
    F: Fn(&serde_json::Value) -> T,
{
    let item = f(&input);
    let output = item.to_value();
    assert_eq!(input, output);
}

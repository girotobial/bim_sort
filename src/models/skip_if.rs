pub fn false_() -> bool {
    false
}

pub fn true_() -> bool {
    true
}

pub fn is_false(x: &bool) -> bool {
    !x
}

pub const fn is_true(b: &bool) -> bool {
    *b
}

pub const fn is_none<T>(option: &Option<T>) -> bool {
    option.is_none()
}

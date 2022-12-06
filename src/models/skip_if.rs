#[must_use]
pub const fn false_() -> bool {
    false
}

#[must_use]
pub const fn true_() -> bool {
    true
}

#[must_use]
pub const fn is_false(x: &bool) -> bool {
    !x
}

#[must_use]
pub const fn is_true(b: &bool) -> bool {
    *b
}

#[must_use]
pub const fn is_none<T>(option: &Option<T>) -> bool {
    option.is_none()
}

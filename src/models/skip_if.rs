#[must_use]
pub const fn false_() -> bool {
    false
}

#[must_use]
pub const fn true_() -> bool {
    true
}

#[allow(clippy::trivially_copy_pass_by_ref)]
#[must_use]
pub fn is_false(x: &bool) -> bool {
    !x
}

#[allow(clippy::trivially_copy_pass_by_ref)]
#[must_use]
pub const fn is_true(b: &bool) -> bool {
    *b
}

#[must_use]
pub const fn is_none<T>(option: &Option<T>) -> bool {
    option.is_none()
}

/*
Bim Sort, sorts bim files for better compatibility with git
Copyright (C) 2022  Alexander Robinson

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

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

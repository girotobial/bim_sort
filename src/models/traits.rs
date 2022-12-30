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

/// Trait for types that allow sorting of sub structs / vectors.
///
/// # How can I implement `RecursiveSort`?
///
/// You must define an implementation of [`recursive_sort`].
///
/// Here's an example
///
/// ```
/// use crate::models::traits::RecursiveSort;
///
///
/// struct Person {
///     id: u32,
///     friends: Vec<'static &str>
/// }
///
/// impl RecursiveSort for Person {
///     fn recursive_sort(&mut self) {
///         self.friends.sort();
///     }
/// }
/// ```
///
/// [`recursive_sort`]: RecursiveSort::recursive_sort
pub trait RecursiveSort {
    /// This method calls sorts any sub fields of the struct it is called on.
    /// Any `Vec` fields must contain items that implment `Ord`.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::models::traits::RecursiveSort;
    ///
    /// struct Person {
    ///     id: u32,
    ///     friends: Vec<'static &str>
    /// }
    ///
    /// impl RecursiveSort for Person {
    ///     fn recursive_sort(&mut self) {
    ///         self.friends.sort();
    ///     }
    /// }
    ///
    /// let mut person = Person {id: 1, friends: vec!["Geoff", "Bob", "Charlie"]}
    /// person.recursive_sort();
    /// assert_eq!(person.friends, vec!["Bob", "Charlie", "Geoff"])
    /// ```
    fn recursive_sort(&mut self);
}

impl<T: RecursiveSort + Ord> RecursiveSort for Vec<T> {
    /// Method to support generic implementations of [`recursive_sort`] for `Vec` of structs that implement `Ord` and `RecursiveSort`.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::models::traits::RecursiveSort;
    ///
    /// struct Database {
    ///     tables: Vec<Table>
    /// }
    ///
    /// impl RecursiveSort for Database {
    ///     fn recursive_sort(&mut self) {
    ///         self.tables.recursive_sort();
    ///     }
    /// }
    ///
    /// struct Table {
    ///     name: String
    ///     columns: Vec<Column>
    /// }
    ///
    /// impl Ord for Table {
    ///     fn cmp(&self, other: &Self) {
    ///         self.name.cmp(&other.name)
    ///     }
    /// }
    ///
    /// impl PartialOrd for Table {
    ///     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    ///         Some(self.cmp(other))
    ///     }
    /// }
    ///
    /// impl RecursiveSort for Table {
    ///     fn recursive_sort(&mut self) {
    ///         self.columns.sort();
    ///     }
    /// }
    ///
    /// #[derive(PartialEq, Eq, PartialOrd, Ord)]
    /// struct Column {
    ///     name: String,
    ///     type_: String,
    /// }
    /// ```
    ///
    /// [`recursive_sort`]: RecursiveSort::recursive_sort
    fn recursive_sort(&mut self) {
        for item in self.iter_mut() {
            item.recursive_sort();
        }
        self.sort();
    }
}

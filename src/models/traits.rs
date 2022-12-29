pub trait RecursiveSort {
    fn recursive_sort(&mut self);
}

impl<T: RecursiveSort + Ord> RecursiveSort for Vec<T> {
    fn recursive_sort(&mut self) {
        for item in self.iter_mut() {
            item.recursive_sort();
        }
        self.sort();
    }
}

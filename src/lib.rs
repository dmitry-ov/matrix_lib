#![allow(dead_code)]

use std::fmt::Debug;
use std::ops::{Add, Mul};
use std::usize;

#[derive(Debug)]
pub struct Matrix<T, const LENGTH: usize> {
    items: [T; LENGTH],
}

impl<T, const LENGTH: usize> Matrix<T, LENGTH> {
    pub fn get_by_index(&self, index: usize) -> &T {
        &self.items[index]
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn get_items(&self) -> &[T; LENGTH] {
        &self.items
    }
}

impl<T: Add<Output = T> + Copy, const LENGTH: usize> Matrix<T, LENGTH> {
    pub fn add_to_each_item(&mut self, value: T) {
        self.items.iter_mut().for_each(|item| *item = *item + value);
    }
}

impl<T: Add<Output = T> + Copy, const LENGTH: usize> Matrix<T, LENGTH> {
    pub fn summary(&self) -> T {
        let refs = self.get_items();
        let length = refs.len();
        let mut summary = refs[0];
        for i in refs.iter().take(length).skip(1) {
            summary = summary + *i;
        }
        summary
    }
}

impl<T: Mul<Output = T> + Copy, const LENGTH: usize> Matrix<T, LENGTH> {
    pub fn mul_to_each_item(&mut self, value: T) {
        self.items.iter_mut().for_each(|item| *item = *item * value);
    }
}

impl<T: Mul<Output = T> + Copy, const LENGTH: usize> Matrix<T, LENGTH> {
    pub fn multiply(&self) -> T {
        let refs = self.get_items();
        let length = refs.len();
        let mut summary = refs[0];
        for i in refs.iter().take(length).skip(1) {
            summary = summary * *i;
        }
        summary
    }
}

#[derive(Debug)]
pub struct MatrixSlice<'a, 'x, T, const LENGTH: usize> {
    refs_array: &'a [&'x Matrix<T, LENGTH>],
}

impl<'a, 'x, T, const LENGTH: usize> MatrixSlice<'a, 'x, T, LENGTH> {
    pub fn get_matrix_by_index(&self, index: usize) -> &'x Matrix<T, LENGTH> {
        self.refs_array[index]
    }
}

impl<'a, 'x, T: Add<Output = T> + Copy, const LENGTH: usize> MatrixSlice<'a, 'x, T, LENGTH> {
    pub fn summary(&self) -> T {
        let length = self.refs_array.len();
        let mut sum = self.refs_array[0].summary();
        for i in 1..length {
            sum = sum + self.refs_array[i].summary();
        }
        sum
    }
}

impl<'a, 'x, T: Mul<Output = T> + Copy, const LENGTH: usize> MatrixSlice<'a, 'x, T, LENGTH> {
    pub fn multiply(&self) -> T {
        let length = self.refs_array.len();
        let mut sum = self.refs_array[0].multiply();
        for i in 1..length {
            sum = sum * self.refs_array[i].multiply();
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_by_index() {
        let a = Matrix::<i32, 5> {
            items: [1, 2, 3, 4, 5],
        };
        assert_eq!(a.get_by_index(0), &1);
    }

    #[test]
    fn size() {
        let a = Matrix::<i32, 5> {
            items: [1, 2, 3, 4, 5],
        };
        assert_eq!(a.size(), 5);
    }

    #[test]
    fn add_to_each_item() {
        let mut a = Matrix::<i32, 3> { items: [1, 2, 3] };
        a.add_to_each_item(1);
        assert_eq!(a.get_items(), &[2, 3, 4]);
    }

    #[test]
    fn matrix_summary() {
        assert_eq!(6, Matrix::<i32, 3> { items: [1, 2, 3] }.summary());
    }

    #[test]
    fn matrix_multiply() {
        assert_eq!(
            120,
            Matrix::<i32, 5> {
                items: [1, 2, 3, 4, 5]
            }
            .multiply()
        );
    }

    #[test]
    fn mul_to_each_item() {
        let mut a = Matrix::<i32, 3> { items: [1, 2, 3] };
        a.mul_to_each_item(2);
        assert_eq!(a.get_items(), &[2, 4, 6]);
    }

    #[test]
    fn get_matrix_by_index() {
        let result_matrix = Matrix::<i32, 5> {
            items: [6, 7, 8, 9, 10],
        };
        let result_matrix_ref: &Matrix<i32, 5> = {
            let t = MatrixSlice::<i32, 5> {
                refs_array: &[
                    &Matrix::<i32, 5> {
                        items: [1, 2, 3, 4, 5],
                    },
                    &result_matrix,
                    &Matrix::<i32, 5> {
                        items: [11, 12, 13, 14, 15],
                    },
                ],
            };
            t.get_matrix_by_index(1)
        };
        assert_eq!(result_matrix_ref.get_items(), result_matrix.get_items());
    }

    #[test]
    fn matrix_slice_summary() {
        let t = MatrixSlice::<i32, 5> {
            refs_array: &[
                &Matrix::<i32, 5> {
                    items: [1, 2, 3, 4, 5],
                },
                &Matrix::<i32, 5> {
                    items: [6, 7, 8, 9, 10],
                },
                &Matrix::<i32, 5> {
                    items: [11, 12, 13, 14, 15],
                },
            ],
        };
        assert_eq!(120, t.summary());
    }

    #[test]
    fn matrix_slice_multiply() {
        let t = MatrixSlice::<i32, 5> {
            refs_array: &[
                &Matrix::<i32, 5> {
                    items: [1, 2, 3, 4, 5],
                },
                &Matrix::<i32, 5> {
                    items: [6, 7, 8, 9, 10],
                },
            ],
        };
        assert_eq!(3_628_800, t.multiply());
    }
}

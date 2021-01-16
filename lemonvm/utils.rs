use core::{
    intrinsics::likely,
    ops::{Index, IndexMut},
};
use std::{fmt::Debug, intrinsics::unlikely};

#[derive(Debug, Clone, Copy)]
pub struct StackPtr<T: Clone + Sized + Debug> {
    ptr: *mut T,
    pub size: usize,
    pub top: usize,
}

impl<T: Clone + Sized + Debug> StackPtr<T> {
    pub fn new_uninit(size: usize) -> StackPtr<T> {
        StackPtr {
            ptr: core::ptr::null_mut(),
            size,
            top: 0,
        }
    }
    pub fn from_ptr(ptr: *mut T, size: usize) -> StackPtr<T> {
        StackPtr { ptr, size, top: 0 }
    }
    pub fn reserve_space_from_current_stack(&mut self, size: usize) -> StackPtr<T> {
        if likely(self.size >= self.top + size) {
            let new_ptr = unsafe { self.ptr.add(self.top) };
            self.top += size;
            Self::from_ptr(new_ptr, size)
        } else {
            panic!("Stack Memory Overflow");
        }
    }
    pub fn remove_space_from_current_stack(&mut self, size: usize) {
        self.top -= size;
    }
    pub fn check_reserve(&self, size: usize) {
        if unlikely(self.size < self.top + size) {
            panic!("Stack Memory Overflow");
        }
    }

    pub fn push(&mut self, value: T) {
        let top = self.top;
        self[top] = value;
        self.top += 1;
    }

    pub fn checked_push(&mut self, value: T) {
        let top = self.top;
        if likely(self.size >= top + 1) {
            self[top] = value;
            self.top += 1;
        } else {
            panic!("Stack Memory Overflow")
        }
    }
}

impl<T: Clone + std::fmt::Debug> Index<usize> for StackPtr<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &*self.ptr.add(index) }
    }
}

impl<T: Clone + std::fmt::Debug> IndexMut<usize> for StackPtr<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut *self.ptr.add(index) }
    }
}

#[test]
fn test_stack_ptr() {
    let mut arr = [1, 2, 3, 4, 5, 6, 7];
    let mut stack_ptr = StackPtr::from_ptr(arr.as_mut_ptr(), 7);
    stack_ptr.checked_push(2);
    assert!(arr[0] == 2);
    stack_ptr.checked_push(3);
    stack_ptr.checked_push(4);
    stack_ptr.checked_push(5);
    stack_ptr.checked_push(6);
    stack_ptr.checked_push(7);
    stack_ptr.checked_push(8);
    assert!(stack_ptr.top == arr.len());
    let err = std::panic::catch_unwind(move || {
        stack_ptr.checked_push(9);
    });
    assert!(!err.is_ok());
}

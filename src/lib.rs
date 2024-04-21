use std::cell::{Cell, UnsafeCell};
use std::marker::PhantomData;

pub struct MutCell<T> where
T: ?Sized, 
{
    not_send: std::marker::PhantomData<*mut T>,
    inner: UnsafeCell<T>,
}
impl<T> MutCell<T> {
    pub fn new(input: T) -> Self {
        MutCell {
            not_send: PhantomData,
            inner: UnsafeCell::new(input),
        }
    }

    // #[inline(always)]
    // pub fn act(&self, action: fn(&mut T)) {
    //     action(unsafe { &mut *self.inner.get() });
    // }

    #[inline(always)]
    pub fn mutate(&self, action: impl FnOnce(&mut T)) {
        action(unsafe { &mut *self.inner.get() });
    }

    #[inline(always)]
    pub fn with<U>(&self, action: impl FnOnce(&mut T) -> U) -> U{
        return action(unsafe { &mut *self.inner.get() });
    }

    #[inline(always)]
    pub fn into_inner(self) -> T {
        return self.inner.into_inner();
    }

    // #[inline(always)]
    // pub fn to_cell(self) -> Cell<T> {
    //     return Cell::new(self.into_inner());
    // }
}

impl<T> MutCell<[T]> {
    
    pub fn as_slice_of_cells(&self) -> &[MutCell<T>] {
        // SAFETY: `MutCell<T>` has the same memory layout as `T`.
        unsafe { &*(self as *const MutCell<[T]> as *const [MutCell<T>]) }
    }
}


impl<T> From<Cell<T>> for MutCell<T> {
    fn from(input: Cell<T>) -> Self {
        MutCell::new(input.into_inner())
    }
}

impl<T> From<MutCell<T>> for Cell<T> {
    fn from(input: MutCell<T>) -> Self {
        Cell::new(input.into_inner())
    }
}

// impl<T> From<MutCell<Vec<T>>> for Vec<MutCell<T>> {
//     fn from(input: MutCell<Vec<T>>) -> Self {
//         unsafe {
//             std::mem::transmute(input)
//         }
//     }
// }
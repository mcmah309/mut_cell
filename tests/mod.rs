use std::cell::{RefCell, UnsafeCell};

use mut_cell::MutCell;

#[test]
fn test() {
    // let x = &Cell::new(NotCopy(1));
    // let mut a = x.get_mut(); // not possible
    // a.0 += 1;

    let x = &RefCell::new(NotCopy(1));
    let mut a = x.borrow_mut();
    (*a).0 += 1;

    let x = &UnsafeCell::new(NotCopy(1));
    let a = x.get();
    unsafe { (*a).0 += 1 }

    // check
    // let x = &MutCell::new(NotCopy(1));
    // let mut y: &mut NotCopy = &mut NotCopy(1);
    // x.act(|a: &mut NotCopy| {
    //     (*a).0 += 1;
    //     y = a;
    // });

    let x = &MutCell::new(NotCopy(1));
    x.with(|a: &mut NotCopy| {
        (*a).0 += 1;
        return true;
    });

    // check
    // x.with(|a: &mut NotCopy| {
    //     (*a).0 += 1;
    //     return a;
    // });

    // UB
    // x.mutate(|a: &mut NotCopy| {
    //     x.mutate(|b: &mut NotCopy| {
    //         (*b).0 += 1;
    //         (*a).0 += 1;
    //     });
    // });

    
}

struct NotCopy(i32);
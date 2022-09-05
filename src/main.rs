use std::cell::Cell;
use std::cell::UnsafeCell;

fn main() {
    fn opaque_read(val: &i32) {
        println!("{}", val);
    }
    
    // unsafe {
    //     let mut data = 10;
    //     let mref1 = &mut data;
    //     let ptr2 = mref1 as *mut i32;
    //     let sref3 = &*mref1;
    //     // let ptr4 = sref3 as *mut i32;
    //     let ptr4 = sref3 as *const i32 as *mut i32;
    
    //     // Random hash of shared reference reads
    //     // opaque_read(sref3);
    //     // opaque_read(sref2);
    //     // opaque_read(sref4);
    //     // opaque_read(sref2);
    //     // opaque_read(sref3);
    
    //     // *mref1 += 1;
    
    //     // opaque_read(&data);
    //     *ptr4 += 4;
    //     opaque_read(sref3);
    //     *ptr2 += 2;
    //     *mref1 += 1;
    
    //     opaque_read(&data);

    // unsafe {
    //     let mut data = 10;
    //     let mref1 = &mut data;
    //     let ptr2 = mref1 as *mut i32;
    //     let sref3 = &*mref1;
    //     let ptr4 = sref3 as *const i32 as *mut i32; 

    //     opaque_read(&*ptr4);
    //     opaque_read(sref3);
    //     *ptr2 += 2;
    //     *mref1 += 1;

    //     opaque_read(&data);
    // }

    // unsafe {
    //     let mut data = 10;
    //     let mref1 = &mut data;
    //     let ptr2 = mref1 as *mut i32;
    //     let sref3 = &*mref1;

    //     *ptr2 += 2;
    //     opaque_read(sref3);
    //     *mref1 += 1;

    //     opaque_read(&data);
    // }

    // unsafe {
    //     let mut data = Cell::new(10);
    //     let mref1 = &mut data;
    //     let ptr2 = mref1 as *mut Cell<i32>;
    //     let sref3 = &* mref1;

    //     sref3.set(sref3.get() + 3);
    //     (*ptr2).set((*ptr2).get() + 2);
    //     mref1.set(mref1.get() + 1);

    //     print!("{}", data.get());

    // }

    // unsafe {
    //     let mut data = UnsafeCell::new(10);
    //     let mref1 = &mut data;
    //     let ptr2 = mref1.get();
    //     let sref3 = &*mref1;

    //     *ptr2 += 2;
    //     opaque_read(&*sref3.get());
    //     *sref3.get() += 3;
    //     *mref1.get() += 1;

    //     println!("{}", *data.get());
    // }

    unsafe {
        let mut data = UnsafeCell::new(10);
        let mref1 = &mut data;
        let sref2 = &*mref1;
        let ptr3 = sref2.get();

        *ptr3 += 3;
        opaque_read(&*sref2.get());
        *sref2.get() += 2;
        *mref1.get() += 1;

        println!("{}", *data.get());
    }

    unsafe {
        let mut data = Box::new(10);
        let ptr1 = (&mut *data) as *mut i32;
    
        // *data += 10;
        *ptr1 += 1;
        *data += 10;
    
        // Should be 21
        println!("{}", data);
    }

    
}
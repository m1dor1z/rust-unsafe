#[test]
fn basic_borrows_unsafe() {
    unsafe {
        let mut data = 10;
        let ref1 = &mut data;
        let ref2 = ref1 as *mut _;

        // *ref1 += 1; // illegal access here, stack borrow. 
        *ref2 += 2;
        *ref1 += 1;
        
        println!("{}", data);
    }

    unsafe {
        let mut data = 10;
        let ref1 = &mut data;
        let ptr2 = ref1 as *mut _;
        let ref3 = &mut *ptr2;
        let ptr4 = ref3 as *mut _;

        *ptr4 += 4;
        *ref3 += 3;
        *ptr2 += 2;
        *ref1 += 1;

        println!("{}", data);
    }
}

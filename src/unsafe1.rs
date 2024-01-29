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

#[test]
fn testing_unsafe_arrays() {
    unsafe {
        let mut data = [0; 10];
        let ref1_at_0 = &mut data[0];
        let ptr2_at_0 = ref1_at_0 as *mut i32;
        let ptr3_at_1 = ptr2_at_0;

        *ptr3_at_1 += 3;
        *ptr2_at_0 += 2;
        *ref1_at_0 += 1;

        println!("{:?}", &data[..]);
    }

    unsafe {
        let mut data = [0; 10];
        let ref1_at_0 = &mut data[0];            
        let ptr2_at_0 = ref1_at_0 as *mut i32;   
        let ptr3_at_0 = ptr2_at_0;               
        let ptr4_at_0 = ptr2_at_0.add(0);        
        let ptr5_at_0 = ptr3_at_0.add(1).sub(1); 

        *ptr3_at_0 += 3;
        *ptr2_at_0 += 2;
        *ptr4_at_0 += 4;
        *ptr5_at_0 += 5;
        *ptr3_at_0 += 3;
        *ptr2_at_0 += 2;
        *ref1_at_0 += 1;

        println!("{:?}", &data[..]);
    }

    unsafe {
        let mut data = [0; 10];
        let slice1 = &mut data[..];

        let (slice2_at_0, slice3_at_1) = slice1.split_at_mut(1);
        let ref4_at_0 = &mut slice2_at_0[0];
        let ref5_at_1 = &mut slice3_at_1[0];
        let ptr6_at_0 = ref4_at_0 as *mut i32;
        let ptr7_at_1 = ref5_at_1 as *mut i32;

        *ptr7_at_1 += 7;
        *ptr6_at_0 += 6;
        *ref5_at_1 += 5;
        *ref4_at_0 += 4;

        println!("{:?}", &data[..]);
    }
}

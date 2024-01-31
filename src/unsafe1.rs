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

    unsafe {
        let mut data = [0; 10];
        let slice1_all = &mut data[..];
        let ptr2_all = slice1_all.as_mut_ptr();

        let ptr3_at_0 = ptr2_all;
        let ptr4_at_1 = ptr2_all.add(1);
        let ref5_at_0 = &mut *ptr3_at_0;
        let ref6_at_1 = &mut *ptr4_at_1;

        *ref6_at_1 += 6;
        *ref5_at_0 += 5;
        *ptr4_at_1 += 4;
        *ptr3_at_0 += 3;

        for idx in 0..10 {
            *ptr2_all.add(idx) += idx;
        }

        for (idx, elem_ref) in slice1_all.iter_mut().enumerate() {
            *elem_ref += idx;
        }

        println!("{:?}", &data[..]);
    }
}

#[test]
fn test_interior_mutability() {
    unsafe {
        let mut data = std::cell::Cell::new(10);
        let mref1 = &mut data;
        let ptr2 = mref1 as *mut std::cell::Cell<i32>;
        let sref3 = &*mref1;

        sref3.set(sref3.get() + 3);
        (*ptr2).set((*ptr2).get() + 2);
        mref1.set(mref1.get() + 1);
        println!("{:?}", data.get());
    }
}

#[test]
fn test_interior_mutability_001() {

    let opaque_read = |val: &i32| {
        println!("{}", val);
    };

    unsafe {
        let mut data = std::cell::UnsafeCell::new(10);
        let mref1 = &mut data;
        let sref2 = &*mref1;
        let ptr3 = sref2.get();             

        *ptr3 += 3;
        opaque_read(&*sref2.get());
        *sref2.get() += 2;
        *mref1.get() += 1;

        println!("{}", *data.get());
    }
}

#[test]
fn test_box_unsafe() {
    unsafe {
        let mut data = Box::new(20);
        let ptr1 = (&mut *data) as *mut i32;

        *ptr1 += 1;
        *data += 20;
        println!("{}", data);
    }
}


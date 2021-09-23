use access_derive::Access;

#[test]
fn get() {
    #[derive(Access)]
    struct Test {
        #[access(get)]
        var1: String
    }

    let test = Test { var1: String::from("this is a test") };
    let a = test.var1();
    assert_eq!(a, "this is a test");
}

#[test]
fn get_with_ref_helper() {
    #[derive(Access)]
    struct Test {
        #[access(get(ref))]
        var1: String
    }

    let test = Test { var1: String::from("this is a test") };
    let a = test.var1();
    assert_eq!(a, "this is a test");
}

#[test]
fn get_with_ref_mut_helper() {
    #[derive(Access)]
    struct Test {
        #[access(get(ref_mut))]
        pub var1: String
    }

    let mut test = Test { var1: String::from("this is a test") };
    let a = test.var1_mut();
    assert_eq!(a, "this is a test");

    a.push_str(" that works!");
    assert_eq!(test.var1, "this is a test that works!");
}

#[test]
fn get_with_ref_mut_and_custom_out_type() {
    #[derive(Access)]
    struct Test {
        #[access(get(ref = "&str"))]
        var1: String
    }

    let test = Test { var1: String::from("this is a test") };
    let a = test.var1();
    assert_eq!(a, "this is a test");
}

#[test]
fn get_with_multiple_helpers() {
    #[derive(Access)]
    struct Test {
        #[access(get(ref = "&str", ref_mut))]
        var1: String
    }

    let mut test = Test { var1: String::from("this is a test") };
    let a = test.var1();
    assert_eq!(a, "this is a test");

    let a = test.var1_mut();
    assert_eq!(a, "this is a test");

    a.push_str(" that works!");
    assert_eq!(test.var1, "this is a test that works!");
}

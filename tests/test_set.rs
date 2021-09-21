use access_derive::Access;

#[test]
fn set() {
    #[derive(Access)]
    struct Test {
        #[access(set)]
        pub var1: String
    }

    let mut test = Test { var1: String::from("this is a test") };

    test.set_var1(String::from("this is a test that works!"));
    assert_eq!(test.var1, "this is a test that works!")
}

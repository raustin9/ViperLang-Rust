define main (argc: i32, argv: String): i32 {
    let i: *i32 = 0;
    let j: [i32] = 0;

    test_function(1, 2 + foo() * 4);

    return 0;
}

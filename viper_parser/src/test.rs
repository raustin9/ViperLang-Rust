#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::Parser;
    use viper_core::source::SourceFile;

    #[test]
    fn parser_simple() {
        let test_file = SourceFile::new_dummy(
            "let i: i32 = 5;", 
            "Test file"
        );
        let file_ptr = Arc::from(test_file);

        let mut parser = Parser::new(&file_ptr);

        parser.parse_top_level().unwrap();
    }
    
    #[test]
    fn parser_return() {
        let test_file = SourceFile::new_dummy(
r#"
define main (argc: i32): i32 {
    let i: i32 = 0;

    0 + 1 * 3 + 3;
    return i + 2;
}
"#, 
            "Test file"
        );
        let file_ptr = Arc::from(test_file);

        let mut parser = Parser::new(&file_ptr);

        parser.parse_top_level().unwrap();
    }
    
    #[test]
    fn parser_yield() {
        let test_file = SourceFile::new_dummy(
r#"
define main (argc: i32): i32 {
    let i: i32 = 0;

    if 0 {
        yield 1;
    }

    return 0;
}
"#, 
            "Test file"
        );
        let file_ptr = Arc::from(test_file);

        let mut parser = Parser::new(&file_ptr);

        parser.parse_top_level().unwrap();
    }
    
    
    #[test]
    fn parser_proc() {
        let test_file = SourceFile::new_dummy(
r#"
define main (argc: i32): i32 {
    let i: i32 = 0;

    0 + 1 * 3 + 3;
}
"#, 
            "Test file"
        );
        let file_ptr = Arc::from(test_file);

        let mut parser = Parser::new(&file_ptr);

        parser.parse_top_level().unwrap();
    }
    
    #[test]
    fn parser_binary() {
        let test_file = SourceFile::new_dummy(
            "let i: i32 = 5 + 2 * -3;", 
            "Test file"
        );
        let file_ptr = Arc::from(test_file);

        let mut parser = Parser::new(&file_ptr);

        parser.parse_top_level().unwrap();
    }
}

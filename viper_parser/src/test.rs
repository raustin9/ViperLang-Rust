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
    fn parser_binary() {
        let test_file = SourceFile::new_dummy(
            "let i: i32 = 5 +2;", 
            "Test file"
        );
        let file_ptr = Arc::from(test_file);

        let mut parser = Parser::new(&file_ptr);

        parser.parse_top_level().unwrap();
    }
}

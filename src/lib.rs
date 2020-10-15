mod core;

#[cfg(test)]
mod tests {

    use crate::core::SemanticExpr;
    #[test]
    fn test_tokenizer() {
        let test_code_1: String = String::from("(+ 10 (+ 1 2 ))");
        assert_eq!(
            SemanticExpr::tokenize(&test_code_1),
            ["(", "+", "10", "(", "+", "1", "2", ")", ")"]
        )
    }
}

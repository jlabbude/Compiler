#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_expr_on_declaratioin() {
        let input = r#"
            func main() {
                let int p = "any";
            }
        "#;
        assert!(run_compiler(&String::from(input)).is_err());
    }
}
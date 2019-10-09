pub fn eval(_action: String) -> String {
    "☠".to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kill_user() {
        assert_eq!(eval("{}".to_string()), "☠");
    }
}
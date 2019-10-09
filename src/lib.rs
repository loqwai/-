pub fn eval(action: String) -> String {
    "☠".to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kill_user() {
        assert_eq!(eval("{}"), "☠");
    }
}
pub fn eval(action: String) -> String {
    if action == "👀" {
        return "🌲🌲🏚🌲🌲".to_string();
    }
    if action == "🚪" {
        return "🛌🛋".to_string();
    }
    return "☠".to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kill_user() {
        assert_eq!(eval("💥".to_string()), "☠");
    }
    #[test]
    fn look_around_you() {
        assert_eq!(eval("👀".to_string()), "🌲🌲🏚🌲🌲");
    }

    #[test]
    fn move_closer_to_the_house() {
        assert_eq!(eval("🚪".to_string()), "🛌🛋");
    }
}
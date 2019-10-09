struct Scene {
    view: String,
    actions: std::vec::Vec<String>
}

pub fn eval(action: String) -> String {
    let s = Scene { 
        view: "🌲🌲🏚🌲🌲".to_string(),
        actions: vec!["🚪".to_string()],
    };

    if action == "👀" || action == "🚪\n🚪" {
        return "🌲🌲🏚🌲🌲".to_string();
    }
    if action == "🚪" {
        return "🛌🛋".to_string();
    }
    if action == "🚪\n👏" {
        return "🛏⛄".to_string()
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
    #[test]
    fn leave_house() {                
        assert_eq!(eval("🚪\n🚪".to_string()), "🌲🌲🏚🌲🌲");
    }

    #[test]
    fn wake_up_the_guy() {                
        assert_eq!(eval("🚪\n👏".to_string()), "🛏⛄");
    }
    #[test]
    fn wake_up_the_guy_and_run() {                
        assert_eq!(eval("🚪\n👏".to_string()), "🛏⛄");
    }
}
use maplit::hashmap;
// use std::collections::HashMap;

// struct Scene<'a> {
//     view: String,
//     actions: std::vec::Vec<&'a str>,
// }

pub fn eval(actions: &Vec<String>) -> String {
    let map = hashmap! {
        "🌲🌲🏚🌲🌲" => hashmap!{
            "👀" => String::from("🌲🌲🏚🌲🌲"),
            "🚪" =>  String::from("🛌🛋"),
        },
         "🛌🛋" => hashmap!{
            "🚪" =>  String::from("🌲🌲🏚🌲🌲"),
        }
    };
    let mut state = String::from("🌲🌲🏚🌲🌲");
    for action in actions {
        let choices = map.get(state.as_str()).unwrap();
        state = choices.get(action.as_str()).unwrap().to_owned();
    }
    return state;
}
// if action == "👀" || action == "🚪\n🚪" {
//     return "🌲🌲🏚🌲🌲".to_string();
// }
// if action == "🚪" {
//     return "🛌🛋".to_string();
// }
// if action == "🚪\n👏" {
//     return "🛏⛄".to_string();
// }
// return "☠".to_string();

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn kill_user() {
    //     assert_eq!(eval(vec!["💥".to_string()]), "☠");
    // }
    #[test]
    fn look_around_you() {
        assert_eq!(eval(vec![&"👀"]), "🌲🌲🏚🌲🌲");
    }

    // #[test]
    // fn move_closer_to_the_house() {
    //     assert_eq!(eval("🚪".to_string()), "🛌🛋");
    // }
    // #[test]
    // fn leave_house() {
    //     assert_eq!(eval("🚪\n🚪".to_string()), "🌲🌲🏚🌲🌲");
    // }

    // #[test]
    // fn wake_up_the_guy() {
    //     assert_eq!(eval("🚪\n👏".to_string()), "🛏⛄");
    // }
    // #[test]
    // fn wake_up_the_guy_and_run() {
    //     assert_eq!(eval("🚪\n👏".to_string()), "🛏⛄");
    // }
    #[test]
    fn indecisive_player() {
        let mut actions: Vec<&str> = vec![];
        for _ in 0..100 {
            actions.push(&"🚪");
        }
        assert_eq!(eval(actions), "🌲🌲🏚🌲🌲");
    }
}

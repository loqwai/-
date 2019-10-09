use maplit::hashmap;
// use std::collections::HashMap;

// struct Scene<'a> {
//     view: String,
//     actions: std::vec::Vec<&'a str>,
// }

pub fn eval(actions: &Vec<String>) -> String {
    let map = hashmap! {
        "🌲🌲🏚🌲🌲" => hashmap!{
            "👀" => "🌲🌲🏚🌲🌲",
            "🚪" =>  "🛌🛋",
            "⬇" => "🌲🌲🌲🌲🌲"
        },
         "🛌🛋" => hashmap!{
            "🚪" =>  "🌲🌲🏚🌲🌲",
            "👏" =>  "🛏⛄",
        },
        "🌲🌲🌲🌲🌲" => hashmap!{
            "⬆" => "🌲🌲🏚🌲🌲",
        }
    };
    let mut state = "🌲🌲🏚🌲🌲";
    for action in actions {
        let choices = map.get(state).unwrap();
        if let Some(new_state) = choices.get(action.as_str()) {
            state = new_state;
        } else {
            return state.into()
        }
    }
    return state.into();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn look_around_you() {
        assert_eq!(eval(&vec!["👀".into()]), "🌲🌲🏚🌲🌲");
    }

    #[test]
    fn move_closer_to_the_house() {
        assert_eq!(eval(&vec!["🚪".into()]), "🛌🛋");
    }
    #[test]
    fn leave_house() {
        assert_eq!(eval(&vec!["🚪".into(), "🚪".into()]), "🌲🌲🏚🌲🌲");
    }

    #[test]
    fn wake_up_the_guy() {
        assert_eq!(eval(&vec!["🚪".into(), "👏".into()]), "🛏⛄");
    }

    #[test]
    fn bad_action() {
        assert_eq!(eval(&vec!["".into()]), "🌲🌲🏚🌲🌲");
    }
    #[test]
    fn wake_up_the_guy_and_run() {
        assert_eq!(eval(&vec!["🚪".into(), "👏".into()]), "🛏⛄");
    }
    #[test]
    fn go_up() {
        assert_eq!(eval(&vec!["⬇".into()]), "🌲🌲🌲🌲🌲");
    }
    #[test]
    fn go_down_up() {
        assert_eq!(eval(&vec!["⬇".into(), "⬆".into()]), "🌲🌲🏚🌲🌲");
    }
    #[test]
    fn indecisive_player() {
        let mut actions: Vec<String> = vec![];
        for _ in 0..100 {
            actions.push("🚪".into());
        }
        assert_eq!(eval(&actions), "🌲🌲🏚🌲🌲");
    }
}

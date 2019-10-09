use maplit::hashmap;
use std::collections::HashMap;

// type Map<'a> = HashMap<&'a str, HashMap<&'a str, &'a str>>;

struct Room<'a> {
    // state: String,
    actions: HashMap<&'a str, &'a str>,
}

pub fn turn(actions: &Vec<String>) -> String {
    let mut state = "🌲🌲🏚🌲🌲";
    let map = new();
    for action in actions {
        let room = map.get(state).unwrap();
        if let Some(new_state) = room.actions.get(action.as_str()) {
            state = new_state;
        } else {
            return state.into()
        }
    }
    return state.into();
}

type Map<'a> = HashMap<&'a str, Room<'a>>;
// "🌲🌲🏚🌲🌲" => hashmap!{
//     "👀" => "🌲🌲🏚🌲🌲",
//     "🚪" =>  "🛌🛋",
//     "⬇" => "🌲🌲🌲🌲🌲"
// },
//     "🛌🛋" => hashmap!{
//     "🚪" =>  "🌲🌲🏚🌲🌲",
//     "👏" =>  "🛌➡🛏⛄",
// },
// "🌲🌲🌲🌲🌲" => hashmap!{
//     "⬆" => "🌲🌲🏚🌲🌲",
// }
fn new<'a>() -> Map<'a> {
    let map = hashmap! {
        "🌲🌲🏚🌲🌲" => Room{actions: hashmap!{
            "👀" => "🌲🌲🏚🌲🌲",
        }},
    };
    return map;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn look_around_you() {
        assert_eq!(turn(&vec!["👀".into()]), "🌲🌲🏚🌲🌲");
    }

    // #[test]
    // fn move_closer_to_the_house() {
    //     assert_eq!(turn(&vec!["🚪".into()]), "🛌🛋");
    // }
    // #[test]
    // fn leave_house() {
    //     assert_eq!(turn(&vec!["🚪".into(), "🚪".into()]), "🌲🌲🏚🌲🌲");
    // }

    // #[test]
    // fn wake_up_the_guy() {
    //     assert_eq!(turn(&vec!["🚪".into(), "👏".into()]), "🛏⛄");
    // }

    // #[test]
    // fn bad_action() {
    //     assert_eq!(turn(&vec!["".into()]), "🌲🌲🏚🌲🌲");
    // }
    // #[test]
    // fn wake_up_the_guy_and_run() {
    //     assert_eq!(turn(&vec!["🚪".into(), "👏".into()]), "🛏⛄");
    // }
    // #[test]
    // fn go_up() {
    //     assert_eq!(turn(&vec!["⬇".into()]), "🌲🌲🌲🌲🌲");
    // }
    // #[test]
    // fn go_down_up() {
    //     assert_eq!(turn(&vec!["⬇".into(), "⬆".into()]), "🌲🌲🏚🌲🌲");
    // }
    // #[test]
    // fn indecisive_player() {
    //     let mut actions: Vec<String> = vec![];        
    //     for _ in 0..100 {
    //         actions.push("🚪".into());
    //     }
    //     assert_eq!(turn(&actions), "🌲🌲🏚🌲🌲");
    // }
    // #[test]
    // fn kill_rats() {        
    //     let actions = &vec![
    //         "🚪".into(),
    //         "👏".into(),
    //         "🚪".into(),
    //         "🚪".into(),            
    //     ];
    //     assert_eq!(turn(actions), "🛏⛄");
    // }
}

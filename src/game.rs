use maplit::hashmap;
use std::collections::HashMap;
use std::option::NoneError;

// type Map<'a> = HashMap<&'a str, HashMap<&'a str, &'a str>>;
#[derive(Clone)]
struct Room<'a> {
    state: String,
    actions: HashMap<&'a str, &'a str>,
}

// #[derive(Debug)]
// struct TurnError{}


// impl std::convert::From<NoneError> for TurnError {
//     fn from(err: NoneError) -> Self {
//         return TurnError{};
//     }
// }


pub fn turn(actions: &Vec<String>) -> Result<String, NoneError> {
    let mut map = new();
    let mut room: Room;
    match map.get("cabin_in_woods") {
        Some(r) => room = r.clone(),
        None => panic!("aaaaaaaahhhhhhhh")
    }
    for action in actions {
        if let Some(mutation) = room.actions.get(action.as_str()) {
            if mutation.starts_with("➡") {
                let r = Room{
                    state: room.state.clone(),
                    actions: room.actions.clone(),
                };
                map.insert("inside_cabin", r);
                continue;
            } else {
                room = map.get(mutation)?.clone();
            }
        } else {
            return Ok(room.state.clone());
        }
    }
    return Ok(room.state.clone());
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
    return hashmap! {
        "cabin_in_woods" => Room{
            state: "🌲🌲🏚🌲🌲".into(),
            actions: hashmap!{
            "👀" => "cabin_in_woods",
            "🚪" => "inside_cabin",
        }},
        "inside_cabin" => Room{
            state: "🛌🛋".into(),
            actions: hashmap!{
            "🚪" => "cabin_in_woods",
            "👏" => "➡🛏⛄"
        }},
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn look_around_you() {
        assert_eq!(turn(&vec!["👀".into()]).unwrap(), "🌲🌲🏚🌲🌲");
    }

    #[test]
    fn move_closer_to_the_house() {
        assert_eq!(turn(&vec!["🚪".into()]).unwrap(), "🛌🛋");
    }
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
    // // }
    
    #[test]
    fn stay_woke() {        
        let actions = &vec![
            "🚪".into(),
            "👏".into(),
            "🚪".into(),
            "🚪".into(),            
        ];
        assert_eq!(turn(actions).unwrap(), "🛏⛄");
    }
}

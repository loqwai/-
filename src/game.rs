use maplit::hashmap;
use std::collections::HashMap;
use std::option::NoneError;

use serde::{Serialize, Deserialize};
#[derive(Clone, Serialize, Deserialize, Debug)]
struct Room {
    state: String,
    actions: HashMap<String, String>,
}

pub fn turn(actions: &Vec<String>) -> Result<String, NoneError> {
    let mut map = new();
    // let mut room: Room;
    let mut room =  &map["cabin_in_woods"];
    let mut current_state = room.state.clone();
    for action in actions {
        match room.actions.get(action) {
            Some(mutation) => {
                // room = next_room(room, mutation);
                if mutation.starts_with("➡") {
                    // let r = Room {
                    //     state: "🛏⛄".into(),
                    //     actions: room.actions.clone(),
                    // };
                    // map.insert(String::from("inside_cabin"), r.clone());
                    // room = r.clone();
                    continue;
                }                
                room = &map[mutation];
                current_state = room.state.clone();

            }
            None => return Ok("⁉".into()),
        }
    }
    return Ok(current_state);
}

fn next_room(room: Room, mutation: &str) -> Room {
    room
}

type Map<'a> = HashMap<String, Room>;

fn new<'a>() -> Map<'a> {
    return hashmap! {
        String::from("cabin_in_woods") => Room{
            state: "🌲🌲🏚🌲🌲".into(),
            actions: hashmap!{
            String::from("👀") => String::from("cabin_in_woods"),
            String::from("🚪") => String::from("inside_cabin"),
            String::from("⬇") => String::from("woods"),
        }},
        String::from("woods") => Room{
            state: "🌲🌲🌲🌲🌲".into(),
            actions: hashmap!{
                String::from("⬆") => String::from("cabin_in_woods"),
            },
        },
        String::from("inside_cabin") => Room{
            state: "🛌🛋".into(),
            actions: hashmap!{
            String::from("🚪") => String::from("cabin_in_woods"),
            String::from("👏") => String::from("➡🛏⛄"),
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
    fn do_something_weird() {
        assert_eq!(turn(&vec!["💃".into()]).unwrap(), "⁉");
    }

    #[test]
    fn move_closer_to_the_house() {
        assert_eq!(turn(&vec!["🚪".into()]).unwrap(), "🛌🛋");
    }
    #[test]
    fn leave_house() {
        assert_eq!(turn(&vec!["🚪".into(), "🚪".into()]).unwrap(), "🌲🌲🏚🌲🌲");
    }

    #[test]
    fn wake_up_the_guy() {
        assert_eq!(turn(&vec!["🚪".into(), "👏".into()]).unwrap(), "🛏⛄");
    }

    #[test]
    fn bad_action() {
        assert_eq!(turn(&vec!["".into()]).unwrap(), "🌲🌲🏚🌲🌲");
    }
    #[test]
    fn wake_up_the_guy_and_run() {
        assert_eq!(turn(&vec!["🚪".into(), "👏".into()]).unwrap(), "🛏⛄");
    }
    #[test]
    fn go_down() {
        assert_eq!(turn(&vec!["⬇".into()]).unwrap(), "🌲🌲🌲🌲🌲");
    }
    #[test]
    fn go_down_up() {
        assert_eq!(turn(&vec!["⬇".into(), "⬆".into()]).unwrap(), "🌲🌲🏚🌲🌲");
    }
    #[test]
    fn indecisive_player() {
        let mut actions: Vec<String> = vec![];
        for _ in 0..100 {
            actions.push("🚪".into());
        }
        assert_eq!(turn(&actions).unwrap(), "🌲🌲🏚🌲🌲");
    }
    #[rustfmt::skip]
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

    // #[test]
    // fn wake_up_put_back_asleep() {
    //     let actions = &vec![
    //         "🚪".into(),
    //         "👏".into(),
    //         "🚪".into(),
    //         "🔨".into(),
    //         "🚪".into(),
    //     ];
    //     assert_eq!(turn(actions).unwrap(), "🛌🛋");
    // }
}

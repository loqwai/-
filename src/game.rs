use maplit::hashmap;
use std::collections::HashMap;
use std::option::NoneError;

#[derive(Clone)]
struct Room {
    state: String,
    actions: HashMap<String, String>,
}

pub fn turn(actions: &Vec<String>) -> Result<String, NoneError> {
    let mut map = new();
    // let mut room: Room;    
    let mut room_name = String::from("cabin_in_woods");
    for action in actions {       
        let room =  &map[&room_name];        
        match room.actions.get(action) {
            Some(mutation) => {                
                // room = next_room(room, mutation);
                if mutation.starts_with("➡") {
                    let r = Room {
                        state: "🛏⛄".into(),
                        actions: room.actions.clone(),
                    };
                    map.insert("inside_cabin".into(), r);
                    continue;
                }                
                room_name = mutation.into();
            }
            None => return Ok("⁉".into()),
        }
    }
    return Ok(map[&room_name].state.clone());
}

type Map<'a> = HashMap<String, Room>;

fn new<'a>() -> Map<'a> {
    return hashmap! {
        "cabin_in_woods".into() => Room{
            state: "🌲🌲🏚🌲🌲".into(),
            actions: hashmap!{
            "👀".into() => "cabin_in_woods".into(),
            "🚪".into() => "inside_cabin".into(),
            "⬇".into() => "woods".into(),
        }},
        "woods".into() => Room{
            state: "🌲🌲🌲🌲🌲".into(),
            actions: hashmap!{
                "⬆".into() => "cabin_in_woods".into(),
            },
        },
        "inside_cabin".into() => Room{
            state: "🛌🛋".into(),
            actions: hashmap!{
            "🚪".into() => "cabin_in_woods".into(),
            "👏".into() => "➡🛏⛄".into(),
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
        assert_eq!(turn(&vec!["".into()]).unwrap(), "⁉");
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

    #[test]
    fn wake_up_put_back_asleep() {
        let actions = &vec![
            "🚪".into(),
            "👏".into(),
            "🚪".into(),
            "🔨".into(),
            "🚪".into(),
        ];
        assert_eq!(turn(actions).unwrap(), "🛌🛋");
    }
}

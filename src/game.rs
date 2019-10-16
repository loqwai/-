use maplit::hashmap;
use std::collections::HashMap;

type Action = |map: Map| -> Map; 

#[derive(Clone)]
struct Room {
    state: String,
    actions: HashMap<String, Action>,
}

pub fn turn(actions: &Vec<String>) -> String {
    let mut map = new();
    // let mut room: Room;    
    let mut room_name = String::from("cabin_in_woods");
    for action in actions {       
        let room =  &map[&room_name];        
        match room.actions.get(action) {
            Some(mutation) => {                
                map = mutation(map);
            }
            None => return format!("{}⁉", action),
        }
    }
    return map.rooms[&room_name].state.clone();
}

fn go_somewhere(room_name: String) -> Action {
    return |map: Map| -> Map {
        Map{
            current_room: room_name.clone(),
            rooms: map.rooms.clone(),
        }
    }
}

struct Map {
    rooms: HashMap<String, Room>
}

fn new() -> Map {
    return Map{
        current_room: "woods",
        rooms: hashmap! {
        "cabin_in_woods".into() => Room{
            state: "🌲🌲🏚🌲🌲".into(),
            actions: hashmap!{
            "👀".into() => go_somewhere("cabin_in_woods".into()),
            "🚪".into() => go_somewhere("inside_cabin".into()),
            "⬇".into() => go_somewhere("woods".into()),
        }},
        "woods".into() => Room{
            state: "🌲🌲🌲🌲🌲".into(),
            actions: hashmap!{
                "⬆".into() => go_somewhere("cabin_in_woods".into()),
            },
        },
        "inside_cabin".into() => Room{
            state: "🛌🛋".into(),
            actions: hashmap!{
            "🚪".into() => go_somewhere("cabin_in_woods".into()),
            "👏".into() => go_somewhere("➡🛏⛄".into()),
            "🔨".into() => go_somewhere("➡🛌🛋".into()),
        }},
    }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn look_around_you() {
        assert_eq!(turn(&vec!["👀".into()]), "🌲🌲🏚🌲🌲");
    }

    #[test]
    fn do_something_weird() {
        assert_eq!(turn(&vec!["💃".into()]), "💃⁉");
    }

    #[test]
    fn move_closer_to_the_house() {
        assert_eq!(turn(&vec!["🚪".into()]), "🛌🛋");
    }
    #[test]
    fn leave_house() {
        assert_eq!(turn(&vec!["🚪".into(), "🚪".into()]), "🌲🌲🏚🌲🌲");
    }

    #[test]
    fn wake_up_the_guy() {
        assert_eq!(turn(&vec!["🚪".into(), "👏".into()]), "🛏⛄");
    }

    #[test]
    fn wake_up_the_guy_and_run() {
        assert_eq!(turn(&vec!["🚪".into(), "👏".into()]), "🛏⛄");
    }
    #[test]
    fn go_down() {
        assert_eq!(turn(&vec!["⬇".into()]), "🌲🌲🌲🌲🌲");
    }
    #[test]
    fn go_down_up() {
        assert_eq!(turn(&vec!["⬇".into(), "⬆".into()]), "🌲🌲🏚🌲🌲");
    }
    #[test]
    fn indecisive_player() {
        let mut actions: Vec<String> = vec![];
        for _ in 0..100 {
            actions.push("🚪".into());
        }
        assert_eq!(turn(&actions), "🌲🌲🏚🌲🌲");
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
        assert_eq!(turn(actions), "🛏⛄");
    }

    #[test]
    fn wake_up_put_back_asleep() {
        let actions = &vec![
            "🚪".into(),
            "👏".into(),
            "🚪".into(),
            "🚪".into(),
            "🔨".into(),
        ];
        assert_eq!(turn(actions), "🛌🛋");
    }
     #[test]
    fn you_cant_hammer() {
        let actions = &vec![
            "🚪".into(),
            "🔨".into(),            
        ];
        assert_eq!(turn(actions), "🔨⁉");
    }
}

use maplit::hashmap;
use std::collections::HashMap;

trait Mutator: MutatorClone {
    fn mutate(&self, map: Map) -> Map;
}

trait MutatorClone {
    fn clone_box(&self) -> Box<dyn Mutator>;
}

impl<T> MutatorClone for T where T: 'static + Mutator + Clone {
    fn clone_box(&self) -> Box<dyn Mutator> {
        return Box::new(self.clone());
    }
}

impl Clone for Box<dyn Mutator> {
    fn clone(&self) -> Box<dyn Mutator> {
        self.clone_box()
    }
}

#[derive(Clone)]
struct SomewhereGoer {
    room_name: String,
}

impl Mutator for SomewhereGoer {
    fn mutate(&self, map: Map) -> Map {
        Map{
            current_room: self.room_name.clone(),
            rooms: map.rooms.clone(),
        }
    }
}

fn go_somewhere(room_name: String) -> Box<dyn Mutator> {
    Box::new(SomewhereGoer {
        room_name: room_name,
    })
}

#[derive(Clone)]
struct Room {
    state: String,
    actions: HashMap<String, Box<dyn Mutator>>,
}

struct Map {
    current_room: String,
    rooms: HashMap<String, Room>
}

fn new() -> Map {
    return Map{
        current_room: "cabin_in_woods".into(),
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

pub fn turn(actions: &Vec<String>) -> String {
    let mut map = new();
    let room_name = map.current_room.clone();
    for action in actions {       
        let room =  &map.rooms[&room_name].clone();        
        match room.actions.get(action) {
            Some(mutation) => {                
                map = mutation.mutate(map);
            }
            None => return format!("{}⁉", action),
        }
    }
    return map.rooms[&room_name].state.clone();
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

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
        Map {
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
struct RoomStateChanger {
    room_name: String,
    new_state: String,
}

impl Mutator for RoomStateChanger {
    fn mutate(&self, map: Map) -> Map {
        let mut rooms = map.rooms.clone();
        let mut room = rooms[&self.room_name].clone();
        room.state = self.new_state.clone();
        rooms.insert(self.room_name.clone(), room);
        
        Map {
            current_room: map.current_room.clone(),
            rooms: rooms,
        }
    }
}

fn change_room_state(room_name: String, new_state: String) -> Box<dyn Mutator> {
    Box::new(RoomStateChanger{
        room_name: room_name,
        new_state: new_state,
    })
}

#[derive(Clone)]
struct RoomActionsChanger {
    room_name: String,
    actions: HashMap<String, Box<dyn Mutator>>,
}

impl Mutator for RoomActionsChanger {
    fn mutate(&self, map: Map) -> Map {
        let mut rooms = map.rooms.clone();
        let mut room = rooms[&self.room_name].clone();
        room.actions = self.actions.clone();
        rooms.insert(self.room_name.clone(), room);
        
        Map {
            current_room: map.current_room.clone(),
            rooms: rooms,
        }
    }
}

fn change_room_actions(room_name: String, actions: HashMap<String, Box<dyn Mutator>>) -> Box<dyn Mutator> {
    Box::new(RoomActionsChanger{room_name: room_name, actions: actions})
}

#[derive(Clone)]
struct CompositeMutator {
    mutators: Vec<Box<dyn Mutator>>,
}

impl Mutator for CompositeMutator {
    fn mutate(&self, map: Map) -> Map {
        let mut current_map = map;

        for mutator in self.mutators.clone() {
            current_map = mutator.mutate(current_map);
        }

        current_map
    }
}

fn compose_mutators(mutators: Vec<Box<dyn Mutator>>) -> Box<dyn Mutator> {
    Box::new(CompositeMutator{mutators: mutators})
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
    Map {
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
                    "👏".into() => compose_mutators(vec![
                        change_room_state("inside_cabin".into(), "🛏⛄".into()),
                        change_room_actions("inside_cabin".into(), hashmap!{
                            "🚪".into() => go_somewhere("cabin_in_woods".into()),
                            "🔨".into() => change_room_state("inside_cabin".into(), "🛌🛋".into()),
                        })
                    ]),
                },
            },
        },
    }
}

pub fn turn(actions: &Vec<String>) -> String {
    let mut map = new();
    for action in actions {       
        let room_name = map.current_room.clone();
        let room =  &map.rooms[&room_name].clone();        
        match room.actions.get(action) {
            Some(mutation) => {                
                map = mutation.mutate(map);
            }
            None => return format!("{}⁉", action),
        }
    }

    map.rooms[&map.current_room].state.clone()
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

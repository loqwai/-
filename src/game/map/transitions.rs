use std::collections::HashMap;

pub trait Transition: TransitionClone {
    fn transit(&self, map: Map) -> Map;
}

pub trait TransitionClone {
    fn clone_box(&self) -> Box<dyn Transition>;
}

impl<T> TransitionClone for T where T: 'static + Transition + Clone {
    fn clone_box(&self) -> Box<dyn Transition> {
        return Box::new(self.clone());
    }
}

impl Clone for Box<dyn Transition> {
    fn clone(&self) -> Box<dyn Transition> {
        self.clone_box()
    }
}

#[derive(Clone)]
struct SomewhereGoer {
    room_name: String,
}

impl Transition for SomewhereGoer {
    fn transit(&self, map: Map) -> Map {
        Map {
            current_room: self.room_name.clone(),
            rooms: map.rooms.clone(),
        }
    }
}

pub fn go_somewhere(room_name: &'static str) -> Box<dyn Transition> {
    Box::new(SomewhereGoer {
        room_name: room_name.into(),
    })
}

#[derive(Clone)]
struct RoomStateChanger {
    room_name: String,
    new_state: String,
}

impl Transition for RoomStateChanger {
    fn transit(&self, map: Map) -> Map {
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

pub fn change_room_state(room_name: &'static str, new_state: &'static str) -> Box<dyn Transition> {
    Box::new(RoomStateChanger{
        room_name: room_name.into(),
        new_state: new_state.into(),
    })
}

#[derive(Clone)]
struct RoomActionsChanger {
    room_name: String,
    actions: HashMap<String, Box<dyn Transition>>,
}

impl Transition for RoomActionsChanger {
    fn transit(&self, map: Map) -> Map {
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

pub fn change_room_actions(room_name: &'static str, actions: HashMap<String, Box<dyn Transition>>) -> Box<dyn Transition> {
    Box::new(RoomActionsChanger{room_name: room_name.into(), actions: actions})
}

#[derive(Clone)]
struct CompositeMutator {
    mutators: Vec<Box<dyn Transition>>,
}

impl Transition for CompositeMutator {
    fn transit(&self, map: Map) -> Map {
        let mut current_map = map;

        for mutator in self.mutators.clone() {
            current_map = mutator.transit(current_map);
        }

        current_map
    }
}

pub fn compose_transitions(mutators: Vec<Box<dyn Transition>>) -> Box<dyn Transition> {
    Box::new(CompositeMutator{mutators: mutators})
}

#[derive(Clone)]
pub struct Room {
    pub state: String,
    pub actions: HashMap<String, Box<dyn Transition>>,
}

pub struct Map {
    pub current_room: String,
    pub rooms: HashMap<String, Room>
}

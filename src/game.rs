use maplit::hashmap;
use std::collections::HashMap;
use std::option::NoneError;

#[derive(Clone)]
struct Room<'a> {
    state: String,
    actions: HashMap<&'a str, &'a str>,
}


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
                    state: "🛏⛄".into(),
                    actions: room.actions.clone(),
                };
                map.insert("inside_cabin", r.clone());
                room = r.clone();
                continue;
            }
            room = map.get(mutation)?.clone();
        } else {
            return Ok(room.state.clone());
        }
    }
    return Ok(room.state.clone());
}

type Map<'a> = HashMap<&'a str, Room<'a>>;
fn new<'a>() -> Map<'a> {
    return hashmap! {
        "cabin_in_woods" => Room{
            state: "🌲🌲🏚🌲🌲".into(),
            actions: hashmap!{
            "👀" => "cabin_in_woods",
            "🚪" => "inside_cabin",
            "⬇" => "woods",
        }},
        "woods" => Room{
            state: "🌲🌲🌲🌲🌲".into(),
            actions: hashmap!{
                "⬆" => "cabin_in_woods",
            },
        },
        "inside_cabin" => Room{
            state: "🛌🛋".into(),
            actions: hashmap!{
            "🚪" => "cabin_in_woods",
            "👏" => "➡🛏⛄",
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

mod map;

pub fn turn(actions: &Vec<&'static str>) -> &'static str {
    let mut map = map::new();
    for action in actions {       
        let room_name = map.current_room.clone();
        let room = map.rooms[room_name].clone();        
        match room.actions.get(action) {
            Some(mutation) => {                
                map = mutation.transit(map);
            }
            None => {
                Box::leak(format!("{}⁉", action).into_boxed_str());
            }
        }
    }

    map.rooms[map.current_room].state.clone()
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
        let mut actions: Vec<&'static str> = vec![];
        for _ in 0..100 {
            actions.push("🚪");
        }
        assert_eq!(turn(&actions), "🌲🌲🏚🌲🌲");
    }
    
    #[test]
    #[rustfmt::skip]
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
    // #[test]
    // fn you_cant_hammer() {
    //     let actions = &vec![
    //         "⬅".into(),
    //         "✋🗡".into(),
    //         "👀".into(),            
    //     ];
    //     assert_eq!(turn(actions), "🌴🏜🏜");
    // }
}

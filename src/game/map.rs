use maplit::hashmap;
mod transitions;

pub fn new() -> transitions::Map {
    transitions::Map {
        current_room: "cabin_in_woods".into(),
        rooms: hashmap! {
            "cabin_in_woods".into() => transitions::Room {
                state: "🌲🌲🏚🌲🌲".into(),
                actions: hashmap!{
                    "👀".into() => transitions::go_somewhere("cabin_in_woods"),
                    "🚪".into() => transitions::go_somewhere("inside_cabin"),
                    "⬇".into() => transitions::go_somewhere("woods"),
                }},
            "woods".into() => transitions::Room{
                state: "🌲🌲🌲🌲🌲".into(),
                actions: hashmap!{
                    "⬆".into() => transitions::go_somewhere("cabin_in_woods"),
                },
            },
            "inside_cabin".into() => transitions::Room{
                state: "🛌🛋".into(),
                actions: hashmap!{
                    "🚪".into() => transitions::go_somewhere("cabin_in_woods"),
                    "👏".into() => transitions::compose_transitions(vec![
                        transitions::change_room_state("inside_cabin", "🛏⛄"),
                        transitions::change_room_actions("inside_cabin", hashmap!{
                            "🚪".into() => transitions::go_somewhere("cabin_in_woods"),
                            "🔨".into() => transitions::change_room_state("inside_cabin", "🛌🛋"),
                        }),
                    ]),
                },
            },
        },
    }
}
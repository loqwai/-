use maplit::hashmap;
mod transitions;

pub fn new() -> transitions::Map {
    transitions::Map {
        current_room: "cabin_in_woods".into(),
        rooms: hashmap! {
            "cabin_in_woods".into() => transitions::Room {
                state: "🌲🌲🏚🌲🌲".into(),
                actions: hashmap!{
                    "👀".into() => transitions::go_somewhere("cabin_in_woods".into()),
                    "🚪".into() => transitions::go_somewhere("inside_cabin".into()),
                    "⬇".into() => transitions::go_somewhere("woods".into()),
                }},
            "woods".into() => transitions::Room{
                state: "🌲🌲🌲🌲🌲".into(),
                actions: hashmap!{
                    "⬆".into() => transitions::go_somewhere("cabin_in_woods".into()),
                },
            },
            "inside_cabin".into() => transitions::Room{
                state: "🛌🛋".into(),
                actions: hashmap!{
                    "🚪".into() => transitions::go_somewhere("cabin_in_woods".into()),
                    "👏".into() => transitions::compose_transitions(vec![
                        transitions::change_room_state("inside_cabin".into(), "🛏⛄".into()),
                        transitions::change_room_actions("inside_cabin".into(), hashmap!{
                            "🚪".into() => transitions::go_somewhere("cabin_in_woods".into()),
                            "🔨".into() => transitions::change_room_state("inside_cabin".into(), "🛌🛋".into()),
                        }),
                    ]),
                },
            },
        },
    }
}
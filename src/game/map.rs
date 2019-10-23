use maplit::hashmap;
mod transitions;

pub fn new() -> transitions::Map {
    transitions::Map {
        current_room: "cabin_in_woods",
        rooms: hashmap! {
            "cabin_in_woods" => transitions::Room {
                state: "🌲🌲🏚🌲🌲",
                actions: hashmap!{
                    "👀" => transitions::go_somewhere("cabin_in_woods"),
                    "🚪" => transitions::go_somewhere("inside_cabin"),
                    "⬇" => transitions::go_somewhere("woods"),
                }},
            "woods" => transitions::Room{
                state: "🌲🌲🌲🌲🌲",
                actions: hashmap!{
                    "⬆" => transitions::go_somewhere("cabin_in_woods"),
                    "⬅" => transitions::go_somewhere("desert_paradise"),
                },
            },
            "inside_cabin" => transitions::Room{
                state: "🛌🛋",
                actions: hashmap!{
                    "🚪" => transitions::go_somewhere("cabin_in_woods"),
                    "👏" => transitions::compose_transitions(vec![
                        transitions::change_room_state("inside_cabin", "🛏⛄"),
                        transitions::change_room_actions("inside_cabin", hashmap!{
                            "🚪" => transitions::go_somewhere("cabin_in_woods"),
                            "🔨" => transitions::change_room_state("inside_cabin", "🛌🛋"),
                        }),
                    ]),
                },
            },
        },
    }
}
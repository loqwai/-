lazy_static!{
static ref map:  HashMap<&'static str, HashMap<&'static str, &'static str>> = {
        let mut m = HashMap::new();
        let mut actions = HashMap::new();
        actions.insert("👀","🌲🌲🏚🌲🌲");
        actions.insert("🚪","🛌🛋");
        m.insert("🌲🌲🏚🌲🌲", actions);
        m
        // m.insert("🛌🛋", actions)
        //   => hashmap!{
        //     "🚪" =>  "🌲🌲🏚🌲🌲",
        // }
    };
}
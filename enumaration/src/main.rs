// // enums are allows definition of types that can be one of several values (variants)
// enum TrafficLight {
//     Red,
//     Yellow,
//     Green,
// }

// // print_light_actio checks a variant of TrafficLight and prints its corresponding result
// fn print_light_action(light: TrafficLight) {
//     match light {
//         TrafficLight::Red => println!("Stop!"),
//         TrafficLight::Yellow => println!("Slow down."),
//         TrafficLight::Green => println!("Go!"),
//     }
// }

// fn main() {
//     let light = TrafficLight::Green;
//     let light2 = TrafficLight::Yellow;
//     let light3 = TrafficLight::Red;
//     print_light_action(light);
//     print_light_action(light2);
//     print_light_action(light3);
// }

#[derive(Debug)]
struct Character {
    name: String,
    health: u32,
    class: CharacterClass,
}

#[derive(Debug)]
enum CharacterClass {
    Warrior { sword_type: String, clan: String },
    Army { kills: u32 },
    Pilot(u8),
}

// impl for enum
impl CharacterClass {
    fn call(&self) {
        if let CharacterClass::Warrior{clan, sword_type} = &self {
            println!("the weapon is {} and the clan is {}", sword_type, clan)
        }
    }
}

fn make_army(kills: u32) -> CharacterClass {
    CharacterClass::Army { kills }
}

fn main() {
    let character1 = Character {
        name: "Roach".to_string(),
        health: 20,
        class: CharacterClass::Warrior {
            clan: "wamoto".to_string(),
            sword_type: "ss".to_string(),
        },
    };

    println!("{}, {}", character1.name, character1.health);

    // we can print one data as follows
    // .. ignores the rest fields in pattern matching
    // we are also borrowing instead of calling character1.class directly
    // cause sword_type and clan are not Copy
    println!(
        "{}",
        match &character1.class {
            CharacterClass::Warrior { sword_type, .. } => sword_type,
            _ => "",
        }
    );

    let empty_string = "".to_string();

    // we can print both data as follows
    // .. ignores the rest fields in pattern matching
    // the format! macro creates and returns a formatted String based on the provided format
    println!(
        "{}",
        match &character1.class {
            CharacterClass::Warrior { sword_type, clan } => format!("{}, {}", sword_type, clan),
            _ => format!("{}, {}", &empty_string, &empty_string), // you can't call "".to_string() directly at this part since it is a
                                                 // temporary value and it will be dropped at the end of the match scope,
                                                 // hence the borrow (&) will be invalid (i.e., from a non-existing memory address)
        }
    );

    // or as follows
    if let CharacterClass::Warrior { sword_type, clan } = &character1.class {
        println!("{sword_type}, {clan}")
    };

    let character2 = Character {
        name: "Price".to_string(),
        health: 20,
        class: CharacterClass::Pilot(33),
    };

    println!("{}", match character2.class {
        CharacterClass::Pilot(xp) => xp,
        _ => 0
    });

    character1.class.call();

    let army1_class = make_army(21);

    let army1 = Character {
        name: "Rock".to_string(),
        health: 100,
        class: army1_class
    };

    // in the following, we are able to call army1.class without borrowing
    // cause the whole army variant data is Copy
    println!("{}", match army1.class {
        CharacterClass::Army { kills } => kills,
        _ => 0
    });

    println!("{}", match army1.class {
        CharacterClass::Army { kills } => kills,
        _ => 0
    });

    // // assuming that the army variant would be... Army { kills: u32, rank: String },
    // // then the following second println will give an error of moved value
    // println!("{}", match army1.class {
    //     CharacterClass::Army { kills, rank } => format!("{}, {}", kills, rank),
    //     _ => format!("{}", 0)
    // });

    // println!("{}", match army1.class {
    //     CharacterClass::Army { kills, rank } => format!("{}, {}", kills, rank),
    //     _ => format!("{}", 0)
    // });
}

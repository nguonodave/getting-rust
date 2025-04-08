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
    Pilot (u8),
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

    println!("{:#?}", character1);

    // we can print one data as follows
    // .. ignores the rest fields in pattern matching
    println!("{}", match &character1.class {
        CharacterClass::Warrior { sword_type, .. } => sword_type,
        _ => "",
    });

    let empty_string = "".to_string();

    // we can print both data as follows
    // .. ignores the rest fields in pattern matching
    println!("{:?}", match &character1.class {
        CharacterClass::Warrior { sword_type, clan } => (sword_type, clan),
        _ => (&empty_string, &empty_string),
    });

    // or as follows
    if let CharacterClass::Warrior{sword_type, clan} = &character1.class {
        println!("{sword_type}, {clan}")
    };

    let character2 = Character {
        name: "Price".to_string(),
        health: 20,
        class: CharacterClass::Pilot(33)
    };
}

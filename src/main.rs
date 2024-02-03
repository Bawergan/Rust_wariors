struct Entity<'a> {
    name: String,
    health: i32,
    strength: i32,
    armor: i32,
    inventory: [Option<&'a Item>; 5],
    hand: Option<&'a Item>,
}

impl Entity<'_> {
    fn recieve_damage(&mut self, dmg_amount: i32) {
        self.health -= dmg_amount;
    }

    fn set_in_hand(&mut self, item: &Item) {
        for item_availvable in self.inventory {
            if item_availvable == Some(item) {
                self.hand = item_availvable;
                return;
            }
        }
    }
}

struct Item {
    name: String,
    damage: i32,
    heal: i32,
}
impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

fn main() {
    let sword = Item {
        name: String::from("sword"),
        damage: 5,
        heal: 0,
    };
    let potion = Item {
        name: String::from("potion"),
        damage: 0,
        heal: 5,
    };

    let mut warior1 = Entity {
        name: String::from("player1"),
        health: 20,
        strength: 5,
        armor: 2,
        inventory: [Some(&sword), None, None, None, None],
        hand: None,
    };

    let mut warior2 = Entity {
        name: String::from("player2"),
        health: 20,
        strength: 5,
        armor: 2,
        inventory: [Some(&potion), None, None, None, None],
        hand: None,
    };

    warior1.set_in_hand(&sword);
    warior2.set_in_hand(&potion);

    let mut move_counter: u64 = 0;
    while warior1.health > 0 && warior2.health > 0 {
        move_counter += 1;
        println!("move: {}", move_counter);

        if move_counter % 2 == 0 {
            do_move(&mut warior1, &mut warior2);
        } else {
            do_move(&mut warior2, &mut warior1);
        }

        println!("{} health {}", warior1.name, warior1.health);
        println!("{} health {}", warior2.name, warior2.health);
    }
}

fn do_move(player1: &mut Entity, player2: &mut Entity) {
    let mut dmg_amount = player1.strength / player2.armor;
    let mut heal_amount = 1;

    match player1.hand {
        Some(hand) => {
            dmg_amount *= hand.damage;
            heal_amount *= hand.heal;
        }
        None => (),
    }

    player2.recieve_damage(dmg_amount);
    player1.recieve_damage(-heal_amount);

    println!(
        "{} -> {}: {} dmg",
        player1.name,
        player2.name,
        dmg_amount,
    );
    println!(
        "{} -> {}: {} heal",
        player1.name,
        player1.name,
        heal_amount,
    );
}

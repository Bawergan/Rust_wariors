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
        self.health -= dmg_amount - self.armor;
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
        inventory: [Some(&sword), None, None, None, None],
        hand: None,
    };

    warior1.set_in_hand(&sword);
    warior2.set_in_hand(&sword);

    let mut move_counter: u64 = 0;
    while warior1.health > 0 && warior2.health > 0 {
        move_counter += 1;
        println!("move: {}", move_counter);

        if move_counter % 2 == 0 {
            match warior2.hand {
                Some(hand) => warior1.recieve_damage(warior2.strength * hand.damage),
                None => warior1.recieve_damage(warior2.strength),
            }
        } else {
            match warior1.hand {
                Some(hand) => warior2.recieve_damage(warior1.strength * hand.damage),
                None => warior2.recieve_damage(warior1.strength),
            }
        }

        println!("{} health {}", warior1.name, warior1.health);
        println!("{} health {}", warior2.name, warior2.health);
    }
}

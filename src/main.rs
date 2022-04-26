use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

const BOARD_SIZE_X: i32 = 17630;
const BOARD_SIZE_Y: i32 = 9000;

#[derive(Debug)]
struct Player {
    id: u16,
    health: u32,
    mana: u32,
    base: Point,
    units: Vec<Unit>
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Debug)]
struct Unit {
    id: u16,
    position: Point,
    shield_life: u8,
    is_controlled: u8,
    health: i32,
    trajectory: Vector,
    near_base: i8,
    threat_for: i8
}

#[derive(Debug)]
struct Vector {
    vx: i32,
    vy: i32
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        (((other.x - self.x).pow(2) + (other.y - self.y).pow(2)) as f64).sqrt()
    }
}

impl Unit {
    fn target_nearest_threat(&self, monsters: &Vec<Unit>) -> Point {
        if monsters.len() == 0 {
            return Point {
                x: self.position.x,
                y: self.position.y
            };
        }

        let nearest_target: &Unit = &monsters[0];

        let mut point = Point {
            x: nearest_target.position.x,
            y: nearest_target.position.y
        };

        let mut distance = f64::INFINITY;

        for monster in monsters {
            if monster.threat_for == 1 {
                let new_distance = self.position.distance(&monster.position);

                if new_distance < distance {
                    point.x = monster.position.x;
                    point.y = monster.position.y;
                    distance = new_distance;
                }
            }
        }

        point
    }
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();

    // The corner of the map representing your base
    let base_x = parse_input!(inputs[0], i32);
    let base_y = parse_input!(inputs[1], i32);

    let mut opponent_base_x = BOARD_SIZE_X;
    let mut opponent_base_y = BOARD_SIZE_Y;

    if base_x == BOARD_SIZE_X && base_y == BOARD_SIZE_Y {
        opponent_base_x = 0;
        opponent_base_y = 0;
    }

    let mut player = Player {
        id: 0,
        health: 3,
        mana: 0,
        base: Point {
            x: base_x,
            y: base_y
        },
        units: vec!()
    };

    let mut opponent = Player {
        id: 1,
        health: 3,
        mana: 0,
        base: Point {
            x: opponent_base_x,
            y: opponent_base_y
        },
        units: vec!()
    };

    let mut monsters : Vec<Unit> = Vec::new();

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();

    // Always 3
    let heroes_per_player = parse_input!(input_line, u8);

    // game loop
    loop {
        for i in 0..2 as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();

            // Your base health
            let health = parse_input!(inputs[0], u32);

            // Ignore in the first league; Spend ten mana to cast a spell
            let mana = parse_input!(inputs[1], u32);

            if i == 0 {
                player.health = health;
                player.mana = mana;
            } else {
                opponent.health = health;
                opponent.mana = mana;
            }
        }

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();

        // Amount of heros and monsters you can see
        let entity_count = parse_input!(input_line, u16);

        monsters.clear();

        for _i in 0..entity_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();

            // Unique identifier
            let id = parse_input!(inputs[0], u16);

            // 0=monster, 1=your hero, 2=opponent hero
            let unit_type = parse_input!(inputs[1], u8);

            // Position of this entity
            let x = parse_input!(inputs[2], i32);
            let y = parse_input!(inputs[3], i32);

            // Ignore for this league; Count down until shield spell fades
            let shield_life = parse_input!(inputs[4], u8);

            // Ignore for this league; Equals 1 when this entity is under a control spell
            let is_controlled = parse_input!(inputs[5], u8);

            // Remaining health of this monster
            let health = parse_input!(inputs[6], i32);

            // Trajectory of this monster
            let vx = parse_input!(inputs[7], i32);
            let vy = parse_input!(inputs[8], i32);

            // 0=monster with no target yet, 1=monster targeting a base
            let near_base = parse_input!(inputs[9], i8);

            // Given this monster's trajectory, is it a threat to 1=your base, 2=your opponent's base, 0=neither
            let threat_for = parse_input!(inputs[10], i8);

            let unit = Unit {
                id: id,
                position: Point {
                    x: x,
                    y: y
                },
                shield_life: shield_life,
                is_controlled: is_controlled,
                health: health,
                trajectory: Vector {
                    vx: vx,
                    vy: vy
                },
                near_base: near_base,
                threat_for: threat_for
            };

            match unit_type {
                0 => monsters.push(unit),
                1 => player.units.push(unit),
                2 => opponent.units.push(unit),
                _ => panic!("")
            }
        }

        // eprintln!("{:?}", player);
        // eprintln!("{:?}", opponent);
        // eprintln!("{:?}", monsters);

        for i in 0..heroes_per_player as usize {
            // In the first league: MOVE <x> <y> | WAIT; In later leagues: | SPELL <spellParams>;
            // println!("WAIT");

            let point = player.units[i].target_nearest_threat(&monsters);
            println!("MOVE {} {}", point.x, point.y);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_distance() {
        let point1 = Point {
            x: 1,
            y: 1
        };

        let point2 = Point {
            x: 10,
            y: 10
        };

        assert_eq!(point1.distance(&point2), 12.727922061357855);
    }
}

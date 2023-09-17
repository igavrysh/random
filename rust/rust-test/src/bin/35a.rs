// Topic: Match guards & binding
//
// Summary:
// * A tile-based game requires different logic for different kinds
//   of tiles. Print different messages depending on the kind of
//   tile selected.
//
// Requirements:
// * Bricks:
//   * Colored bricks should print "The brick color is [color]"
//   * Other bricks should print "[Bricktype] brick"
// * Water:
//   * Pressure levels 10 and over should print "High water pressure!"
//   * Pressure levels under 10 should print "Water pressure level: [Pressure]"
// * Grass, Dirt, and Sand should all print "Ground tile"
// * Treasure Chests:
//   * If the treasure is Gold and the amount is at least 100, print "Lots of gold!"
// * Everything else shoud not print any messages
//
// Notes:
// * Use a single match expression utilizing guards to implement the program
// * Run the program and print the messages with at least 4 different tiles

#[derive(Debug)]
enum TreasureItem {
    Gold,
    SuperPower,
}

#[derive(Debug)]
struct TreasureChest {
    content: TreasureItem,
    amount: usize,
}

#[derive(Debug)]
struct Pressure(u16);

#[derive(Debug)]
enum BrickStyle {
    Dungeon,
    Gray,
    Red,
}

#[derive(Debug)]
enum Tile {
    Brick(BrickStyle),
    Dirt,
    Grass,
    Sand,
    Treasure(TreasureChest),
    Water(Pressure),
    Wood,
}

fn tile_desc(t: &Tile) -> Option<String> {
    match t {
        Tile::Brick(s @ BrickStyle::Gray | s @ BrickStyle::Red) 
            => Some(format!("The brick color: {:?}", s)),
        Tile::Brick(s) 
            => Some(format!("[{:?}] brick", s)),
        Tile::Water(Pressure(p)) if *p >= 10 
            => Some("High water pressure!".to_owned()),
        Tile::Water(Pressure(p)) 
            => Some(format!("Water pressure level: [{}]", p)),
        Tile::Grass | Tile::Dirt | Tile::Sand 
            => Some(String::from("Ground tile")),
        Tile::Treasure(TreasureChest{content: TreasureItem::Gold, amount}) if *amount >= 100
            => Some("Lots of gold!".to_string()),
        _ => None,
    }
}

fn main() {
    let tiles = vec!(
        Tile::Brick(BrickStyle::Red), 
        Tile::Treasure(TreasureChest { content: TreasureItem::Gold, amount: 120 }),
        Tile::Water(Pressure(12)),
        Tile::Dirt,
        Tile::Treasure(TreasureChest { content: TreasureItem::SuperPower, amount: 42 }),
        Tile::Grass,
        Tile::Wood,
        Tile::Sand,
        Tile::Brick(BrickStyle::Dungeon),
        Tile::Brick(BrickStyle::Gray),
    );

    tiles.iter().for_each(|t| {
        match tile_desc(t) {
            Some(s) => println!("{}", s),
            _ => ()
        }

    })
}

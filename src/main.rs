use std::cmp;
use tcod::colors::*;
use tcod::console::*;

//Size of the Window
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

//Size of the Map
const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;

//The Map is a Vec of Vectors of Tiles
//with the type Keyword we can pass map and not Vec<Vec<Tile>> as type
type Map = Vec<Vec<Tile>>;
struct Game {
    map: Map,
}

// A rectangle on the map, used to characterise a room
#[derive(Clone, Copy, Debug)]
struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

//Rectangle stores the coordinates for the top-left and bottom-right points
impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }
}

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150,
};

const LIMIT_FPS: i32 = 20; //20 frames-per-second max

struct Tcod {
    root: Root,
    con: Offscreen,
}

//This is a generic object: player, moster, item
//it's always represented by a character on the screen
//to not hav the vars everywhere we define an object
//We don’t want the Copy behaviour for Object (we could accidentally modify a copy instead of the original and get our changes lost for example), but Debug is useful, so let’s add the Debug derive to our Object as well:
#[derive(Debug)]
struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color,
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        Object { x, y, char, color }
    }

    //movement
    pub fn move_by(&mut self, dx: i32, dy: i32, game: &Game) {
        if !game.map[(self.x + dx) as usize][(self.y + dy) as usize].blocked {
            self.x += dx;
            self.y += dy;
        }
    }

    //set the color and then draw the character that represents thisobject at its position
    //the dyn keyword highlights that Console is a trait and not a concrete type (such as a struct)
    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
}

//Tile struct, so we have the ability to define some structs later on
//The #[derive(…​)] bit automatically implements certain behaviours (Rust calls them traits, other languages use interfaces) you list there.
//Debug is to let us print the Tile’s contents and Clone and Copy will let us copy the values on assignment or function call instead of moving them. So they’ll behave like e.g. integers in this respect.
#[derive(Clone, Copy, Debug)]
struct Tile {
    blocked: bool,
    block_sight: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
        }
    }

    pub fn wall() -> Self {
        Tile {
            blocked: true,
            block_sight: true,
        }
    }
}

//takes in the rect and places it in the map
fn create_room(room: Rect, map: &mut Map) {
    //go through the tiles in the rectangle and make them passable
    //the A..B notation specifies a range that's inclusive at the beginning but exclusive at the end
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            map[x as usize][y as usize] = Tile::empty();
        }
    }
}

//&mut = borrowing the values, otherwise it would be consumed by the first handle_keys call
fn handle_keys(tcod: &mut Tcod, game: &Game, player: &mut Object) -> bool {
    //for keyinput
    use tcod::input::Key;
    use tcod::input::KeyCode::*;
    //true = exit game, false = stay in game
    let key = tcod.root.wait_for_keypress(true);

    match key {
        Key {
            code: Enter,
            alt: true,
            ..
        } => {
            //Alt+Enter: toggle fullscreen
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
        }
        Key { code: Escape, .. } => return true, //exit game
        //key movements
        //.. means i don't care about the others fileds
        Key { code: Up, .. } => player.move_by(0, -1, game),
        Key { code: Down, .. } => player.move_by(0, 1, game),
        Key { code: Left, .. } => player.move_by(-1, 0, game),
        Key { code: Right, .. } => player.move_by(1, 0, game),

        _ => {}
    }
    false
}

//creating the map
fn make_map() -> Map {
    //fill map with "unblocked" tiles
    //vec! is a shortcut and creates a vector and fills it with random values
    //z.B. vec!['a',42] creates a Vector containing the letter 'a' 42 times
    let mut map = vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    //create two rooms
    let room1 = Rect::new(20, 15, 10, 15);
    let room2 = Rect::new(50, 15, 10, 15);
    create_room(room1, &mut map);
    create_room(room2, &mut map);
    //creating a tunnel
    create_h_tunnel(25, 55, 23, &mut map);

    map
}

//creating a tunnel between rooms
fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
    //horizontal tunnel. 'min()' and 'max()' are used in case 'x1 > x2'
    //We use min and max to make sure the .. range always starts with the smaller of the numbers
    for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map) {
    // vertical tunnel
    for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

//render function that renders the objects and the map
fn render_all(tcod: &mut Tcod, game: &Game, objects: &[Object]) {
    //Draw all objects in the list
    for object in objects {
        object.draw(&mut tcod.con);
    }
    //going through all the tiles and and set their background color
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let wall = game.map[x as usize][y as usize].block_sight;
            if wall {
                tcod.con
                    .set_char_background(x, y, COLOR_DARK_WALL, BackgroundFlag::Set);
            } else {
                tcod.con
                    .set_char_background(x, y, COLOR_DARK_GROUND, BackgroundFlag::Set);
            }
        }
    }

    //blit call from the main function
    blit(
        &tcod.con,
        (0, 0),
        (MAP_WIDTH, MAP_HEIGHT),
        &mut tcod.root,
        (0, 0),
        1.0,
        1.0,
    );
}

fn main() {
    //Setting up the window
    //If setting type specifically let root: Root = Root::initalizer().....init();
    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcod tutorial")
        .init();

    //creating a offline console
    let con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);

    let mut tcod = Tcod { root, con };

    //setting up the fps limit
    //tcod::system::set_fps(LIMIT_FPS);

    //create object representing the player
    let player = Object::new(25, 23, '@', WHITE);
    //create an NPC
    let npc = Object::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2, '@', YELLOW);
    //the list of objects with those two
    let mut objects = [player, npc];

    //creating th egame Object
    let game = Game {
        //generate map (not draw to the scren jet)
        map: make_map(),
    };

    //Adding a Game Loop
    //window_closed() returns true if the window was closed, else false
    while !tcod.root.window_closed() {
        //everything will be drawn white unless said otherwise
        //tcod.con.set_default_foreground(WHITE);
        //clear everything we draw last frame
        tcod.con.clear();
        //drawing the @ on the screen
        //@ needs to be in '' because it needs a char and not a str type
        //BackgroundFlage::None says to ignore the default background
        //tcod.con.put_char(player_x, player_y, '@', BackgroundFlag::None);
        //blit takes a lot of parameters but the usage is pretty straight forward!
        /*
        We take the console we want to blit from (con), the coordinates where to start and the width and height of the area
        we want to build. Then the destination (root), where to start blitting (top left) and finally the foreground
        and background transparency (0.0 fully transparent vs 1.0 completly opaque).
        */

        //now rendering all the object with a for loop
        for object in &objects {
            object.draw(&mut tcod.con);
        }

        //rendering the objects
        render_all(&mut tcod, &game, &objects);
        //drawing everything onto the screen
        tcod.root.flush();
        //we also need to call wait_for_keypress even though we’re not processing keyboard input yet. This is because libtcod handles the window manager’s events (including your request to close the window) in the input processing code.
        //If we didn’t call it, window_close would not work properly and the game would crash or hang.
        //handling input
        let player = &mut objects[0];
        let exit = handle_keys(&mut tcod, &game, player);
        if exit {
            break;
        }
    }
}

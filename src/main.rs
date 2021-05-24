use tcod::colors::*;
use tcod::console::*;

//Size of the Window
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const LIMIT_FPS: i32 = 20; //20 frames-per-second max

struct Tcod {
    root: Root,
    con: Offscreen,
}

//This is a generic object: player, moster, item
//it's always represented by a character on the screen
//to not hav the vars everywhere we define an object
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
    pub fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    //set the color and then draw the character that represents thisobject at its position
    //the dyn keyword highlights that Console is a trait and not a concrete type (such as a struct)
    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
}

//&mut = borrowing the values, otherwise it would be consumed by the first handle_keys call
fn handle_keys(tcod: &mut Tcod, player: &mut Object) -> bool {
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
        Key { code: Up, .. } => player.move_by(0, -1),
        Key { code: Down, .. } => player.move_by(0, 1),
        Key { code: Left, .. } => player.move_by(-1, 0),
        Key { code: Right, .. } => player.move_by(1, 0),

        _ => {}
    }
    false
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
    let con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut tcod = Tcod { root, con };

    //setting up the fps limit
    //tcod::system::set_fps(LIMIT_FPS);

    //create object representing the player
    let player = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', WHITE);
    //create an NPC
    let npc = Object::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2, '@', YELLOW);
    //the list of objects with those two
    let mut objects = [player, npc];

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

        blit(
            &tcod.con,
            (0, 0),
            (SCREEN_WIDTH, SCREEN_HEIGHT),
            &mut tcod.root,
            (0, 0),
            1.0,
            1.0,
        );

        //drawing everything onto the screen
        tcod.root.flush();
        //we also need to call wait_for_keypress even though we’re not processing keyboard input yet. This is because libtcod handles the window manager’s events (including your request to close the window) in the input processing code.
        //If we didn’t call it, window_close would not work properly and the game would crash or hang.
        //handling input
        let player = &mut objects[0];
        let exit = handle_keys(&mut tcod, player);
        if exit {
            break;
        }
    }
}

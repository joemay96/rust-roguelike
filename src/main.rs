use tcod::colors::*;
use tcod::console::*;

//Size of the Window
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const LIMIT_FPS: i32 = 20; //20 frames-per-second max

struct Tcod {
    root: Root,
}

//&mut = borrowing the values, otherwise it would be consumed by the first handle_keys call
fn handle_keys(tcod: &mut Tcod, player_x: &mut i32, player_y: &mut i32) -> bool {
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
        Key { code: Up, .. } => *player_y -= 1,
        Key { code: Down, .. } => *player_y += 1,
        Key { code: Left, .. } => *player_x -= 1,
        Key { code: Right, .. } => *player_x += 1,

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

    let mut tcod = Tcod { root };

    //setting up the fps limit
    //tcod::system::set_fps(LIMIT_FPS);

    //PLayer Coordinates
    let mut player_x = SCREEN_WIDTH / 2;
    let mut player_y = SCREEN_HEIGHT / 2;

    //Adding a Game Loop
    //window_closed() returns true if the window was closed, else false
    while !tcod.root.window_closed() {
        //everything will be drawn white unless said otherwise
        tcod.root.set_default_foreground(WHITE);
        //clear everything we draw last frame
        tcod.root.clear();
        //drawing the @ on the screen
        //@ needs to be in '' because it needs a char and not a str type
        //BackgroundFlage::None says to ignore the default background
        tcod.root
            .put_char(player_x, player_y, '@', BackgroundFlag::None);
        //drawing everything onto the screen
        tcod.root.flush();
        //we also need to call wait_for_keypress even though we’re not processing keyboard input yet. This is because libtcod handles the window manager’s events (including your request to close the window) in the input processing code.
        //If we didn’t call it, window_close would not work properly and the game would crash or hang.
        //handling input
        let exit = handle_keys(&mut tcod, &mut player_x, &mut player_y);
        if exit {
            break;
        }
    }
}

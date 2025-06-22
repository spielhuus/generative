#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
    pub x: f32,      // Rectangle top-left corner position x
    pub y: f32,      // Rectangle top-left corner position y
    pub width: f32,  // Rectangle width
    pub height: f32, // Rectangle height
}

impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Camera3D {
    pub position: Vector3,
    pub target: Vector3,
    pub up: Vector3,
    pub fovy: f32,
    pub projection: i32,
}

impl Camera3D {
    pub fn new(
        position: Vector3,
        target: Vector3,
        up: Vector3,
        fovy: f32,
        projection: i32,
    ) -> Self {
        Self {
            position,
            target,
            up,
            fovy,
            projection,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

pub const WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
    a: 255,
};

pub const GREY: Color = Color {
    r: 200,
    g: 200,
    b: 200,
    a: 200,
};

pub const RED: Color = Color {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};

pub const GREEN: Color = Color {
    r: 0,
    g: 255,
    b: 0,
    a: 255,
};

pub const BLACK: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
};

pub enum KeyboardKey {
    KeyNull = 0, // Key: NULL, used for no key pressed
    // Alphanumeric keys
    Keyapostrophe = 39,   // Key: '
    KeyComma = 44,        // Key: ,
    KeyMinus = 45,        // Key: -
    KeyPeriod = 46,       // Key: .
    KeySlash = 47,        // Key: /
    KeyZero = 48,         // Key: 0
    KeyOne = 49,          // Key: 1
    KeyTwo = 50,          // Key: 2
    KeyThree = 51,        // Key: 3
    KeyFour = 52,         // Key: 4
    KeyFive = 53,         // Key: 5
    KeySix = 54,          // Key: 6
    KeySeven = 55,        // Key: 7
    KeyEight = 56,        // Key: 8
    KeyNine = 57,         // Key: 9
    KeySemicolon = 59,    // Key: ;
    KeyEqual = 61,        // Key: =
    KeyA = 65,            // Key: A | a
    KeyB = 66,            // Key: B | b
    KeyC = 67,            // Key: C | c
    KeyD = 68,            // Key: D | d
    KeyE = 69,            // Key: E | e
    KeyF = 70,            // Key: F | f
    KeyG = 71,            // Key: G | g
    KeyH = 72,            // Key: H | h
    KeyI = 73,            // Key: I | i
    KeyJ = 74,            // Key: J | j
    KeyK = 75,            // Key: K | k
    KeyL = 76,            // Key: L | l
    KeyM = 77,            // Key: M | m
    KeyN = 78,            // Key: N | n
    KeyO = 79,            // Key: O | o
    KeyP = 80,            // Key: P | p
    KeyQ = 81,            // Key: Q | q
    KeyR = 82,            // Key: R | r
    KeyS = 83,            // Key: S | s
    KeyT = 84,            // Key: T | t
    KeyU = 85,            // Key: U | u
    KeyV = 86,            // Key: V | v
    KeyW = 87,            // Key: W | w
    KeyX = 88,            // Key: X | x
    KeyY = 89,            // Key: Y | y
    KeyZ = 90,            // Key: Z | z
    KeyLeftBracket = 91,  // Key: [
    KeyBackslash = 92,    // Key: '\'
    KeyRightBracket = 93, // Key: ]
    KeyGrave = 96,        // Key: `
    // Function keys
    KeySpace = 32,         // Key: Space
    KeyEscape = 256,       // Key: Esc
    KeyEnter = 257,        // Key: Enter
    KeyTab = 258,          // Key: Tab
    KeyBackspace = 259,    // Key: Backspace
    KeyInsert = 260,       // Key: Ins
    KeyDelete = 261,       // Key: Del
    KeyRight = 262,        // Key: Cursor right
    KeyLeft = 263,         // Key: Cursor left
    KeyDown = 264,         // Key: Cursor down
    KeyUp = 265,           // Key: Cursor up
    KeyPageUp = 266,       // Key: Page up
    KeyPageDown = 267,     // Key: Page down
    KeyHome = 268,         // Key: Home
    KeyEnd = 269,          // Key: End
    KeyCapsLock = 280,     // Key: Caps lock
    KeyScrollLock = 281,   // Key: Scroll down
    KeyNumLock = 282,      // Key: Num lock
    KeyPrintScreen = 283,  // Key: Print screen
    KeyPause = 284,        // Key: Pause
    KeyF1 = 290,           // Key: F1
    KeyF2 = 291,           // Key: F2
    KeyF3 = 292,           // Key: F3
    KeyF4 = 293,           // Key: F4
    KeyF5 = 294,           // Key: F5
    KeyF6 = 295,           // Key: F6
    KeyF7 = 296,           // Key: F7
    KeyF8 = 297,           // Key: F8
    KeyF9 = 298,           // Key: F9
    KeyF10 = 299,          // Key: F10
    KeyF11 = 300,          // Key: F11
    KeyF12 = 301,          // Key: F12
    KeyLeftShift = 340,    // Key: Shift left
    KeyLeftControl = 341,  // Key: Control left
    KeyLeftAlt = 342,      // Key: Alt left
    KeyLeftSuper = 343,    // Key: Super left
    KeyRightShift = 344,   // Key: Shift right
    KeyRightControl = 345, // Key: Control right
    KeyRightAlt = 346,     // Key: Alt right
    KeyRightSuper = 347,   // Key: Super right
    KeyKbMenu = 348,       // Key: KB menu
    // Keypad keys
    KeyKp0 = 320,        // Key: Keypad 0
    KeyKp1 = 321,        // Key: Keypad 1
    KeyKp2 = 322,        // Key: Keypad 2
    KeyKp3 = 323,        // Key: Keypad 3
    KeyKp4 = 324,        // Key: Keypad 4
    KeyKp5 = 325,        // Key: Keypad 5
    KeyKp6 = 326,        // Key: Keypad 6
    KeyKp7 = 327,        // Key: Keypad 7
    KeyKp8 = 328,        // Key: Keypad 8
    KeyKp9 = 329,        // Key: Keypad 9
    KeyKpSecimal = 330,  // Key: Keypad .
    KeyKpDivide = 331,   // Key: Keypad /
    KeyKpMultiply = 332, // Key: Keypad *
    KeyKpSubtract = 333, // Key: Keypad -
    KeyKpAdd = 334,      // Key: Keypad +
    KeyKpEnter = 335,    // Key: Keypad Enter
    KeyKpEqual = 336,    // Key: Keypad =
    // Android key buttons
    KeyBack = 4,        // Key: Android back button
    KeyMenu = 5,        // Key: Android menu button
    KeyVolumeUp = 24,   // Key: Android volume up button
    KeyVolumeDown = 25, // Key: Android volume down button
}

#[allow(non_snake_case)]
pub fn IsKeyPressed(key: KeyboardKey) -> bool {
    unsafe { RayIsKeyPressed(key as i32) }
}
#[allow(non_snake_case)]
pub fn IsKeyDown(key: KeyboardKey) -> bool {
    unsafe { RayIsKeyDown(key as i32) }
}

unsafe extern "C" {
    pub fn BeginDrawing();
    pub fn EndDrawing();
    pub fn InitWindow(width: i32, height: i32, title: *const ::std::os::raw::c_char);
    pub fn CloseWindow();
    pub fn SetTargetFPS(frames: i32);
    pub fn Vector2Distance(v1: Vector2, v2: Vector2) -> f32;
    pub fn GetScreenWidth() -> i32;
    pub fn GetScreenHeight() -> i32;
    pub fn WindowShouldClose() -> bool;
    pub fn ClearBackground(color: Color);
    pub fn DrawLine(start_x: i32, start_y: i32, end_x: i32, end_y: i32, color: Color);
    pub fn DrawCircle(center_x: i32, center_y: i32, radius: f32, color: Color);
    pub fn DrawCircleV(v: Vector2, radius: f32, color: Color);
    pub fn DrawRectangle(x: i32, y: i32, width: i32, height: i32, color: Color);
    pub fn DrawRectanglePro(rec: Rectangle, origin: Vector2, rotation: f32, color: Color);
    pub fn DrawRectangleRec(rec: Rectangle, color: Color);
    pub fn CheckCollisionRecs(rec1: Rectangle, rec2: Rectangle) -> bool;
    pub fn DrawText(
        text: *const ::std::os::raw::c_char,
        x: i32,
        y: i32,
        fontSize: i32,
        color: Color,
    );
    #[link_name = "IsKeyPressed"]
    pub fn RayIsKeyPressed(key: i32) -> bool;
    #[link_name = "IsKeyDown"]
    pub fn RayIsKeyDown(key: i32) -> bool;
}

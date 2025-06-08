// pub fn InitWindow(width: i32, height: i32, title: &str) {
//     let c_title = CString::new(title).expect("Invalid title");
//     unsafe {
//         ffi::InitWindow(width, height, c_title.as_ptr());
//     }
// }

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct Color {
    /// Color red value
    pub r: u8,
    /// Color green value
    pub g: u8,
    /// Color blue value
    pub b: u8,
    /// Color alpha value
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

// impl Into<ffi::Camera3D> for Camera3D {
//     fn into(self) -> ffi::Camera3D {
//         unsafe { std::mem::transmute(self) }
//     }
// }

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

// impl Into<ffi::Color> for Color {
//     fn into(self) -> ffi::Color {
//         unsafe { std::mem::transmute(self) }
//     }
// }

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

// impl Into<ffi::Vector3> for Vector3 {
//     fn into(self) -> ffi::Vector3 {
//         unsafe { std::mem::transmute(self) }
//     }
// }

// pub const fn translate(x: f32, y: f32, z: f32) -> ffi::Matrix {
//     ffi::Matrix {
//         m0: 1.0,
//         m4: 0.0,
//         m8: 0.0,
//         m12: x,
//         m1: 0.0,
//         m5: 1.0,
//         m9: 0.0,
//         m13: y,
//         m2: 0.0,
//         m6: 0.0,
//         m10: 1.0,
//         m14: z,
//         m3: 0.0,
//         m7: 0.0,
//         m11: 0.0,
//         m15: 1.0,
//     }
// }
//
pub const WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
    a: 255,
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
}

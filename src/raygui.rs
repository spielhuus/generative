use crate::raylib::Rectangle;

pub enum GuiControl {
    // Default -> populates to all controls when set
    Default = 0,

    // Basic controls
    Label, // Used also for: LABELBUTTON
    Button,
    Toggle, // Used also for: TOGGLEGROUP
    Slider, // Used also for: SLIDERBAR, TOGGLESLIDER
    Progressbar,
    Checkbox,
    Combobox,
    Dropdownbox,
    Textbox, // Used also for: TEXTBOXMULTI
    Valuebox,
    Control11,
    Listview,
    Colorpicker,
    Scrollbar,
    Statusbar,
}

// Gui base properties for every control
// NOTE: RAYGUI_MAX_PROPS_BASE properties (by default 16 properties)
#[derive(Clone)]
pub enum GuiControlProperty {
    BorderColorNormal = 0, // Control border color in STATE_NORMAL
    BaseColorNormal,       // Control base color in STATE_NORMAL
    TextColorNormal,       // Control text color in STATE_NORMAL
    BorderColorFocused,    // Control border color in STATE_FOCUSED
    BaseColorFocused,      // Control base color in STATE_FOCUSED
    TextColorFocused,      // Control text color in STATE_FOCUSED
    BorderColorPressed,    // Control border color in STATE_PRESSED
    BaseColorPressed,      // Control base color in STATE_PRESSED
    TextColorPressed,      // Control text color in STATE_PRESSED
    BorderColorDisabled,   // Control border color in STATE_DISABLED
    BaseColorDisabled,     // Control base color in STATE_DISABLED
    TextColorDisabled,     // Control text color in STATE_DISABLED
    BorderWidth = 12,      // Control border size, 0 for no border
    //TEXT_SIZE,                  // Control text size (glyphs max height) -> GLOBAL for all controls
    //TEXT_SPACING,               // Control text spacing between glyphs -> GLOBAL for all controls
    //TEXT_LINE_SPACING,          // Control text spacing between lines -> GLOBAL for all controls
    TextPadding = 13,   // Control text padding, not considering border
    TextAlignment = 14, // Control text horizontal alignment inside control text bound (after border and padding)
    //TEXT_WRAP_MODE              // Control text wrap-mode inside text bounds -> GLOBAL for all controls
    // }
    //
    // pub enum GuiDefaultProperty {
    TextSize = 16,         // Text size (glyphs max height)
    TextSpacing,           // Text spacing between glyphs
    LineColor,             // Line control color
    BackgroundColor,       // Background color
    TextLineSpacing,       // Text spacing between lines
    TextAlignmentVertical, // Text vertical alignment inside text bounds (after border and padding)
    TextWrapMode,          // Text wrap-mode inside text bounds
                           //TEXT_DECORATION             // Text decoration: 0-None, 1-Underline, 2-Line-through, 3-Overline
                           //TEXT_DECORATION_THICK       // Text decoration line thickness
}

#[allow(non_snake_case)]
pub fn GuiSetStyle(control: GuiControl, property: GuiControlProperty, value: i32) {
    unsafe {
        RayGuiSetStyle(control as i32, property as i32, value);
    }
}

unsafe extern "C" {
    pub fn GuiMessageBox(
        bounds: Rectangle,
        title: *const ::std::os::raw::c_char,
        message: *const ::std::os::raw::c_char,
        buttons: *const ::std::os::raw::c_char,
    ) -> i32;
    pub fn GuiTextBox(
        bounds: Rectangle,
        text: *mut ::std::os::raw::c_char,
        textSize: i32,
        editMode: bool,
    ) -> i32;
    pub fn GuiButton(bounds: Rectangle, text: *const ::std::os::raw::c_char) -> i32;
    #[link_name = "GuiSetStyle"]
    pub fn RayGuiSetStyle(control: i32, property: i32, value: i32);
    pub fn GuiComboBox(
        bounds: Rectangle,
        text: *const ::std::os::raw::c_char,
        active: *mut i32,
    ) -> i32;
    pub fn GuiSlider(
        bounds: Rectangle,
        textLeft: *const ::std::os::raw::c_char,
        textRight: *const ::std::os::raw::c_char,
        value: &f32,
        minValue: f32,
        maxValue: f32,
    ) -> i32;
    pub fn GuiCheckBox(
        bounds: Rectangle,
        text: *const ::std::os::raw::c_char,
        checked: &bool,
    ) -> i32;
}

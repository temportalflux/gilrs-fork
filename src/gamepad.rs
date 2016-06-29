use platform;

use std::mem;

#[derive(Debug)]
pub struct Gilrs {
    gilrs: platform::Gilrs,
    gamepads: Vec<Gamepad>,
}

impl Gilrs {
    pub fn new() -> Self {
        let mut p_gilrs = platform::Gilrs::new();
        let gamepads = p_gilrs.gamepads.drain(0..).map(|gp| Gamepad::new(gp)).collect();
        Gilrs { gilrs: p_gilrs, gamepads: gamepads }
    }

    pub fn pool_events(&mut self) -> EventIterator {
        EventIterator(&mut self.gamepads[0])
    }

    pub fn gamepad(&self, n: usize) -> Option<&Gamepad> {
        self.gamepads.get(n)
    }
}

#[derive(Debug)]
pub struct Gamepad {
    gamepad: platform::Gamepad,
    state: GamepadState,
}

impl Gamepad {
    fn new(gamepad: platform::Gamepad) -> Self {
        Gamepad {
            gamepad: gamepad,
            state: GamepadState::new(),
        }
    }

    pub fn name(&self) -> &String {
        &self.gamepad.name
    }

    pub fn state(&self) -> &GamepadState {
        &self.state
    }

    pub fn is_pressed(&self, btn: Button) -> bool {
        match btn {
            Button::South => self.state.btn_south,
            Button::East => self.state.btn_east,
            Button::North => self.state.btn_north,
            Button::West => self.state.btn_west,
            Button::C => self.state.btn_c,
            Button::Z => self.state.btn_z,

            Button::LeftTrigger => self.state.left_trigger != 0.0,
            Button::LeftTrigger2 => self.state.left_trigger2 != 0.0,
            Button::RightTrigger => self.state.right_trigger != 0.0,
            Button::RightTrigger2 => self.state.right_trigger2 != 0.0,

            Button::Select => self.state.btn_select,
            Button::Start => self.state.btn_start,
            Button::Mode => self.state.btn_mode,

            Button::LeftThumb => self.state.btn_left_thumb,
            Button::RightThumb => self.state.btn_right_thumb,

            Button::DPadUp => self.state.dpad_up,
            Button::DPadDown => self.state.dpad_down,
            Button::DPadLeft => self.state.dpad_left,
            Button::DPadRight => self.state.dpad_right,
        }
    }

    pub fn axis_val(&self, axis: Axis) -> f32 {
        match axis {
            Axis::LeftStickX => self.state.left_stick.0,
            Axis::LeftStickY => self.state.left_stick.1,
            Axis::RightStickX => self.state.right_stick.0,
            Axis::RightStickY => self.state.right_stick.1,
            Axis::LeftTrigger => self.state.left_trigger,
            Axis::LeftTrigger2 => self.state.left_trigger2,
            Axis::RightTrigger => self.state.right_trigger,
            Axis::RightTrigger2 => self.state.right_trigger2,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GamepadState {
    // sticks
    pub right_stick: (f32, f32),
    pub left_stick: (f32, f32),
    pub btn_left_thumb: bool,
    pub btn_right_thumb: bool,
    // triggers
    pub right_trigger: f32,
    pub right_trigger2: f32,
    pub left_trigger: f32,
    pub left_trigger2: f32,
    // action pad
    pub btn_south: bool,
    pub btn_east: bool,
    pub btn_north: bool,
    pub btn_west: bool,
    pub btn_c: bool,
    pub btn_z: bool,
    // menu pad
    pub btn_select: bool,
    pub btn_start: bool,
    pub btn_mode: bool,
    // dpad
    pub dpad_up: bool,
    pub dpad_down: bool,
    pub dpad_left: bool,
    pub dpad_right: bool,
}

impl GamepadState {
    pub fn new() -> Self {
        unsafe { mem::zeroed() }
    }

    fn set_btn(&mut self, btn: Button, val: bool) {
        match btn {
            Button::South => self.btn_south = val,
            Button::East => self.btn_east = val,
            Button::North => self.btn_north = val,
            Button::West => self.btn_west = val,
            Button::C => self.btn_c = val,
            Button::Z => self.btn_z = val,

            Button::LeftTrigger => self.left_trigger = if val { 1.0 } else { 0.0 },
            Button::LeftTrigger2 => self.left_trigger2 = if val { 1.0 } else { 0.0 },
            Button::RightTrigger => self.right_trigger = if val { 1.0 } else { 0.0 },
            Button::RightTrigger2 => self.right_trigger2 = if val { 1.0 } else { 0.0 },

            Button::Select => self.btn_select = val,
            Button::Start => self.btn_start = val,
            Button::Mode => self.btn_mode = val,

            Button::LeftThumb => self.btn_left_thumb = val,
            Button::RightThumb => self.btn_right_thumb = val,

            Button::DPadUp => self.dpad_up = val,
            Button::DPadDown => self.dpad_down = val,
            Button::DPadLeft => self.dpad_left = val,
            Button::DPadRight => self.dpad_right = val,
        };
    }

    fn set_axis(&mut self, axis: Axis, val: f32) {
        match axis {
            Axis::LeftStickX => self.left_stick.0 = val,
            Axis::LeftStickY => self.left_stick.1 = val,
            Axis::RightStickX => self.right_stick.0 = val,
            Axis::RightStickY => self.right_stick.1 = val,
            Axis::LeftTrigger => self.left_trigger = val,
            Axis::LeftTrigger2 => self.left_trigger2 = val,
            Axis::RightTrigger => self.right_trigger = val,
            Axis::RightTrigger2 => self.right_trigger2 = val,
        };
    }
}

pub struct EventIterator<'a>(&'a mut Gamepad);

impl<'a> Iterator for EventIterator<'a> {
    type Item = Event;

    fn next(&mut self) -> Option<Event> {
        self.0.gamepad.event().map(|ev| {
            match ev {
                Event::ButtonPressed(btn) => self.0.state.set_btn(btn, true),
                Event::ButtonReleased(btn) => self.0.state.set_btn(btn, false),
                Event::AxisChanged(axis, val) => self.0.state.set_axis(axis, val),
            }
            ev
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    ButtonPressed(Button),
    ButtonReleased(Button),
    AxisChanged(Axis, f32),
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Button {
    // Action Pad
    South = BTN_SOUTH,
    East = BTN_EAST,
    North = BTN_NORTH,
    West = BTN_WEST,
    C = BTN_C,
    Z = BTN_Z,
    // Triggers
    LeftTrigger = BTN_TL,
    LeftTrigger2 = BTN_TL2,
    RightTrigger = BTN_TR,
    RightTrigger2 = BTN_TR2,
    // Menu Pad
    Select = BTN_SELECT,
    Start = BTN_START,
    Mode = BTN_MODE,
    // Sticks
    LeftThumb = BTN_THUMBL,
    RightThumb = BTN_THUMBR,
    // D-Pad
    DPadUp = BTN_DPAD_UP,
    DPadDown = BTN_DPAD_DOWN,
    DPadLeft = BTN_DPAD_LEFT,
    DPadRight = BTN_DPAD_RIGHT,
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Axis {
    LeftStickX = ABS_X,
    LeftStickY = ABS_Y,
    RightStickX = ABS_RX,
    RightStickY = ABS_RY,
    LeftTrigger = ABS_HAT1Y,
    LeftTrigger2 = ABS_HAT2Y,
    RightTrigger = ABS_HAT1X,
    RightTrigger2 = ABS_HAT2X,
}

// Move this to platform::linux
const BTN_SOUTH: u16 = 0x130;
const BTN_EAST: u16 = 0x131;
const BTN_C: u16 = 0x132;
const BTN_NORTH: u16 = 0x133;
const BTN_WEST: u16 = 0x134;
const BTN_Z: u16 = 0x135;
const BTN_TL: u16 = 0x136;
const BTN_TR: u16 = 0x137;
const BTN_TL2: u16 = 0x138;
const BTN_TR2: u16 = 0x139;
const BTN_SELECT: u16 = 0x13a;
const BTN_START: u16 = 0x13b;
const BTN_MODE: u16 = 0x13c;
const BTN_THUMBL: u16 = 0x13d;
const BTN_THUMBR: u16 = 0x13e;

const BTN_DPAD_UP: u16 = 0x220;
const BTN_DPAD_DOWN: u16 = 0x221;
const BTN_DPAD_LEFT: u16 = 0x222;
const BTN_DPAD_RIGHT: u16 = 0x223;

const ABS_X: u16 = 0x00;
const ABS_Y: u16 = 0x01;
const ABS_RX: u16 = 0x03;
const ABS_RY: u16 = 0x04;
const ABS_HAT1X: u16 = 0x12;
const ABS_HAT1Y: u16 = 0x13;
const ABS_HAT2X: u16 = 0x14;
const ABS_HAT2Y: u16 = 0x15;

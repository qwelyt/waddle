// TODO: Let the matrix hold Enums that are either a Key or a Function.
// If key, then we just check the key value, add it to mods or keycodes.
// If function, then we run the function, sending in the state. This is probably chaning the
// state of the leds or something like that. Maybe change the layout on the fly... hmm
// [ layer
//   [row
//     [col
//       KeyCode(A) / Function(state -> doShit())
//     ]
//   ]
// ]

use K::norde::se;
use Key::KeyCode;

use crate::keycode::K;
use crate::keycode::K::layer;
use crate::layout::Key::Function;
use crate::state::State;

enum Key {
    KeyCode(u8),
    Function(fn(&mut State)),
}

pub const ROWS: usize = 4;
pub const COLS: usize = 12;
pub const LAYERS: usize = 3;
pub const LEDS: usize = 3;
pub const MATRIX: [[[Key; COLS]; ROWS]; LAYERS] = [
    [
        [KeyCode(K::TAB), KeyCode(K::Q), KeyCode(K::W), KeyCode(K::E), KeyCode(K::R), KeyCode(K::T), KeyCode(K::Y), KeyCode(K::U), KeyCode(K::I), KeyCode(K::O), KeyCode(K::P), KeyCode(se::Å), ],
        [KeyCode(K::ESC), KeyCode(K::A), KeyCode(K::S), KeyCode(K::D), KeyCode(K::F), KeyCode(K::G), KeyCode(K::H), KeyCode(K::J), KeyCode(K::K), KeyCode(K::L), KeyCode(se::Ö), KeyCode(se::Ä), ],
        [KeyCode(K::L_SHFT), KeyCode(K::Z), KeyCode(K::X), KeyCode(K::C), KeyCode(K::V), KeyCode(K::B), KeyCode(K::N), KeyCode(K::M), KeyCode(K::COMMA), KeyCode(K::DOT), KeyCode(se::DASH), KeyCode(K::R_SHFT), ],
        [KeyCode(K::L_CTRL), KeyCode(K::L_SUPR), KeyCode(K::BS_N_PIPE), KeyCode(K::L_ALT), Function(l1), KeyCode(K::SPACE), KeyCode(K::RETURN), Function(l2), KeyCode(K::R_ALT), KeyCode(K::MENU), KeyCode(K::R_SUPR), KeyCode(K::R_CTRL), ],
    ],
    [
        [KeyCode(K::TAB), KeyCode(K::Q), KeyCode(K::W), KeyCode(K::E), KeyCode(K::R), KeyCode(K::T), KeyCode(K::Y), KeyCode(K::U), KeyCode(K::I), KeyCode(K::O), KeyCode(K::P), KeyCode(se::Å), ],
        [KeyCode(K::ESC), KeyCode(K::A), KeyCode(K::S), KeyCode(K::D), KeyCode(K::F), KeyCode(K::G), KeyCode(K::H), KeyCode(K::J), KeyCode(K::K), KeyCode(K::L), KeyCode(se::Ö), KeyCode(se::Ä), ],
        [KeyCode(K::L_SHFT), KeyCode(K::Z), KeyCode(K::X), KeyCode(K::C), KeyCode(K::V), KeyCode(K::B), KeyCode(K::N), KeyCode(K::M), KeyCode(K::COMMA), KeyCode(K::DOT), KeyCode(se::DASH), KeyCode(K::R_SHFT), ],
        [KeyCode(K::L_CTRL), KeyCode(K::L_SUPR), KeyCode(K::BS_N_PIPE), KeyCode(K::L_ALT), KeyCode(layer::LOWER), KeyCode(K::SPACE), KeyCode(K::RETURN), KeyCode(layer::RAISE), KeyCode(K::R_ALT), KeyCode(K::MENU), KeyCode(K::R_SUPR), KeyCode(K::R_CTRL), ],
    ],
    [
        [KeyCode(K::TAB), KeyCode(K::Q), KeyCode(K::W), KeyCode(K::E), KeyCode(K::R), KeyCode(K::T), KeyCode(K::Y), KeyCode(K::U), KeyCode(K::I), KeyCode(K::O), KeyCode(K::P), KeyCode(se::Å), ],
        [KeyCode(K::ESC), KeyCode(K::A), KeyCode(K::S), KeyCode(K::D), KeyCode(K::F), KeyCode(K::G), KeyCode(K::H), KeyCode(K::J), KeyCode(K::K), KeyCode(K::L), KeyCode(se::Ö), KeyCode(se::Ä), ],
        [KeyCode(K::L_SHFT), KeyCode(K::Z), KeyCode(K::X), KeyCode(K::C), KeyCode(K::V), KeyCode(K::B), KeyCode(K::N), KeyCode(K::M), KeyCode(K::COMMA), KeyCode(K::DOT), KeyCode(se::DASH), KeyCode(K::R_SHFT), ],
        [KeyCode(K::L_CTRL), KeyCode(K::L_SUPR), KeyCode(K::BS_N_PIPE), KeyCode(K::L_ALT), KeyCode(layer::LOWER), KeyCode(K::SPACE), KeyCode(K::RETURN), KeyCode(layer::RAISE), KeyCode(K::R_ALT), KeyCode(K::MENU), KeyCode(K::R_SUPR), KeyCode(K::R_CTRL), ],
    ],
];


pub fn mo(l: u8, state: &mut State) -> fn(&mut State) {
    |s| { layer(l, s) }
}

pub fn l1(state: &mut State) {
    mo(1, state)(state)
}

pub fn l2(state: &mut State) {
    mo(2, state)(state)
}

pub fn layer(layer: u8, state: &mut State) {
    state.layer_mo(layer)
}
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

use k::norde::se;
use Key::KeyCode;

use crate::keycode::k;
use crate::keycode::k::layer;
use crate::layout::Key::{Function, LayerMo};
use crate::state::State;

pub enum Key {
    KeyCode(u8),
    Function(fn(&mut State)),
    LayerMo(u8),
    LayerCh(u8),
}

pub const ROWS: usize = 4;
pub const COLS: usize = 12;
pub const LAYERS: usize = 3;
pub const LEDS: usize = 3;
pub const MATRIX: [[[Key; COLS]; ROWS]; LAYERS] = [
    [
        [KeyCode(k::TAB), KeyCode(k::Q), KeyCode(k::W), KeyCode(k::E), KeyCode(k::R), KeyCode(k::T), KeyCode(k::Y), KeyCode(k::U), KeyCode(k::I), KeyCode(k::O), KeyCode(k::P), KeyCode(se::Å), ],
        [KeyCode(k::ESC), KeyCode(k::A), KeyCode(k::S), KeyCode(k::D), KeyCode(k::F), KeyCode(k::G), KeyCode(k::H), KeyCode(k::J), KeyCode(k::K), KeyCode(k::L), KeyCode(se::Ö), KeyCode(se::Ä), ],
        [KeyCode(k::L_SHFT), KeyCode(k::Z), KeyCode(k::X), KeyCode(k::C), KeyCode(k::V), KeyCode(k::B), KeyCode(k::N), KeyCode(k::M), KeyCode(k::COMMA), KeyCode(k::DOT), KeyCode(se::DASH), KeyCode(k::R_SHFT), ],
        [KeyCode(k::L_CTRL), KeyCode(k::L_SUPR), KeyCode(k::BS_N_PIPE), KeyCode(k::L_ALT), LayerMo(1), KeyCode(k::SPACE), KeyCode(k::RETURN), LayerMo(2), KeyCode(k::R_ALT), KeyCode(k::MENU), KeyCode(k::R_SUPR), KeyCode(k::R_CTRL), ],
    ],
    [
        [KeyCode(k::TAB), KeyCode(k::Q), KeyCode(k::W), KeyCode(k::E), KeyCode(k::R), KeyCode(k::T), KeyCode(k::Y), KeyCode(k::U), KeyCode(k::I), KeyCode(k::O), KeyCode(k::P), KeyCode(se::Å), ],
        [KeyCode(k::ESC), KeyCode(k::A), KeyCode(k::S), KeyCode(k::D), KeyCode(k::F), KeyCode(k::G), KeyCode(k::H), KeyCode(k::J), KeyCode(k::K), KeyCode(k::L), KeyCode(se::Ö), KeyCode(se::Ä), ],
        [KeyCode(k::L_SHFT), KeyCode(k::Z), KeyCode(k::X), KeyCode(k::C), KeyCode(k::V), KeyCode(k::B), KeyCode(k::N), KeyCode(k::M), KeyCode(k::COMMA), KeyCode(k::DOT), KeyCode(se::DASH), KeyCode(k::R_SHFT), ],
        [KeyCode(k::L_CTRL), KeyCode(k::L_SUPR), KeyCode(k::BS_N_PIPE), KeyCode(k::L_ALT), KeyCode(layer::LOWER), KeyCode(k::SPACE), KeyCode(k::RETURN), KeyCode(layer::RAISE), KeyCode(k::R_ALT), KeyCode(k::MENU), KeyCode(k::R_SUPR), KeyCode(k::R_CTRL), ],
    ],
    [
        [KeyCode(k::TAB), KeyCode(k::Q), KeyCode(k::W), KeyCode(k::E), KeyCode(k::R), KeyCode(k::T), KeyCode(k::Y), KeyCode(k::U), KeyCode(k::I), KeyCode(k::O), KeyCode(k::P), KeyCode(se::Å), ],
        [KeyCode(k::ESC), KeyCode(k::A), KeyCode(k::S), KeyCode(k::D), KeyCode(k::F), KeyCode(k::G), KeyCode(k::H), KeyCode(k::J), KeyCode(k::K), KeyCode(k::L), KeyCode(se::Ö), KeyCode(se::Ä), ],
        [KeyCode(k::L_SHFT), KeyCode(k::Z), KeyCode(k::X), KeyCode(k::C), KeyCode(k::V), KeyCode(k::B), KeyCode(k::N), KeyCode(k::M), KeyCode(k::COMMA), KeyCode(k::DOT), KeyCode(se::DASH), KeyCode(k::R_SHFT), ],
        [KeyCode(k::L_CTRL), KeyCode(k::L_SUPR), KeyCode(k::BS_N_PIPE), KeyCode(k::L_ALT), KeyCode(layer::LOWER), KeyCode(k::SPACE), KeyCode(k::RETURN), KeyCode(layer::RAISE), KeyCode(k::R_ALT), KeyCode(k::MENU), KeyCode(k::R_SUPR), KeyCode(k::R_CTRL), ],
    ],
];
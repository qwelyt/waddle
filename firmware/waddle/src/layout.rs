use k::norde::se;
use Key::{Dead, KeyCode, LayerCh};

use crate::keycode::k;
use crate::keycode::k::layer;
use crate::layout::Key::{Function, LayerMo};
use crate::position::position::Position;
use crate::state::State;

pub enum Key {
    KeyCode(u8),
    Function(fn(&mut State)),
    LayerMo(u8),
    LayerCh(u8),
    Dead,
}

pub const ROWS: usize = 3;
pub const COLS: usize = 12;
pub const BUTTONS: usize = ROWS * COLS;
pub const NUM_CHUNKS: usize = BUTTONS / 6;
pub const LAYERS: usize = 1;
pub const LEDS: usize = 2;
// pub const MATRIX: [[[Key; COLS]; ROWS]; LAYERS] = [
//     [
//         [KeyCode(k::A), KeyCode(k::B)],
//         [KeyCode(k::C), LayerMo(1)],
//     ],
//     [
//         [KeyCode(k::R_SHFT), KeyCode(k::E)],
//         [KeyCode(k::F), Dead],
//     ],
// ];
pub const MATRIX: [[[Key; COLS]; ROWS]; LAYERS] = [
    [
        [KeyCode(k::TAB), KeyCode(k::Q), KeyCode(k::W), KeyCode(k::E), KeyCode(k::R), KeyCode(k::T), KeyCode(k::Y), KeyCode(k::U), KeyCode(k::I), KeyCode(k::O), KeyCode(k::P), KeyCode(se::Å), ],
        [KeyCode(k::ESC), KeyCode(k::A), KeyCode(k::S), KeyCode(k::D), KeyCode(k::F), KeyCode(k::G), KeyCode(k::H), KeyCode(k::J), KeyCode(k::K), KeyCode(k::L), KeyCode(se::Ö), KeyCode(se::Ä), ],
        [KeyCode(k::L_SHFT), KeyCode(k::Z), KeyCode(k::X), KeyCode(k::C), KeyCode(k::V), KeyCode(k::B), KeyCode(k::N), KeyCode(k::M), KeyCode(k::COMMA), KeyCode(k::DOT), KeyCode(se::DASH), KeyCode(k::R_SHFT), ],
        // [KeyCode(k::L_CTRL), KeyCode(k::L_SUPR), KeyCode(k::BS_N_PIPE), KeyCode(k::L_ALT), LayerMo(1), KeyCode(k::SPACE), KeyCode(k::RETURN), LayerMo(2), KeyCode(k::R_ALT), KeyCode(k::MENU), KeyCode(k::R_SUPR), KeyCode(k::R_CTRL), ],
    ],
    // [
    //     [KeyCode(k::TAB), KeyCode(k::Q), KeyCode(k::W), KeyCode(k::E), KeyCode(k::R), KeyCode(k::T), KeyCode(k::Y), KeyCode(k::U), KeyCode(k::I), KeyCode(k::O), KeyCode(k::P), KeyCode(se::Å), ],
    //     [KeyCode(k::ESC), KeyCode(k::A), KeyCode(k::S), KeyCode(k::D), KeyCode(k::F), KeyCode(k::G), KeyCode(k::H), KeyCode(k::J), KeyCode(k::K), KeyCode(k::L), KeyCode(se::Ö), KeyCode(se::Ä), ],
    //     [KeyCode(k::L_SHFT), KeyCode(k::Z), KeyCode(k::X), KeyCode(k::C), KeyCode(k::V), KeyCode(k::B), KeyCode(k::N), KeyCode(k::M), KeyCode(k::COMMA), KeyCode(k::DOT), KeyCode(se::DASH), KeyCode(k::R_SHFT), ],
    //     // [KeyCode(k::L_CTRL), KeyCode(k::L_SUPR), KeyCode(k::BS_N_PIPE), KeyCode(k::L_ALT), Dead, KeyCode(k::SPACE), KeyCode(k::RETURN), Dead, KeyCode(k::R_ALT), KeyCode(k::MENU), KeyCode(k::R_SUPR), KeyCode(k::R_CTRL), ],
    // ],
    // [
    //     [KeyCode(k::TAB), KeyCode(k::Q), KeyCode(k::W), KeyCode(k::E), KeyCode(k::R), KeyCode(k::T), KeyCode(k::Y), KeyCode(k::U), KeyCode(k::I), KeyCode(k::O), KeyCode(k::P), KeyCode(se::Å), ],
    //     [KeyCode(k::ESC), KeyCode(k::A), KeyCode(k::S), KeyCode(k::D), KeyCode(k::F), KeyCode(k::G), KeyCode(k::H), KeyCode(k::J), KeyCode(k::K), KeyCode(k::L), KeyCode(se::Ö), KeyCode(se::Ä), ],
    //     [KeyCode(k::L_SHFT), KeyCode(k::Z), KeyCode(k::X), KeyCode(k::C), KeyCode(k::V), KeyCode(k::B), KeyCode(k::N), KeyCode(k::M), KeyCode(k::COMMA), KeyCode(k::DOT), KeyCode(se::DASH), KeyCode(k::R_SHFT), ],
    //     // [KeyCode(k::L_CTRL), KeyCode(k::L_SUPR), KeyCode(k::BS_N_PIPE), KeyCode(k::L_ALT), Dead, KeyCode(k::SPACE), KeyCode(k::RETURN), Dead, KeyCode(k::R_ALT), KeyCode(k::MENU), KeyCode(k::R_SUPR), KeyCode(k::R_CTRL), ],
    // ],
];
pub static LAYOUT: Layout = Layout { matrix: MATRIX };

pub struct Layout {
    matrix: [[[Key; COLS]; ROWS]; LAYERS],
}


impl Layout {
    pub fn new() -> Self {
        Self { matrix: MATRIX }
    }
    pub fn get_layer_mod(&self, position: &Position) -> u8 {
        match self.matrix[0][position.row() as usize][position.col() as usize] {
            KeyCode(_) => { 0 }
            Function(_) => { 0 }
            LayerMo(l) => { l }
            LayerCh(_) => { 0 }
            Dead => { 0 }
        }
    }

    pub fn get_keycode(&self, layer: u8, position: &Position) -> Option<u8> {
        match self.matrix.get(layer as usize)
            .and_then(|layer| layer.get(position.row() as usize))
            .and_then(|row| row.get(position.col() as usize))
            .unwrap_or(&Dead) {
            KeyCode(kc) => { Some(*kc) }
            Function(_) => { None }
            LayerMo(_) => { None }
            LayerCh(_) => { None }
            Dead => { None }
        }
    }

    pub fn get_mod(&self, layer: u8, position: &Position) -> Option<u8> {
        self.get_keycode(layer, position).filter(k::is_mod).map(k::to_mod_bitfield)
    }

    pub fn get_non_mod(&self, layet: u8, position: &Position) -> Option<u8> {
        self.get_keycode(layet, position).filter(|u| !k::is_mod(u))
    }
}
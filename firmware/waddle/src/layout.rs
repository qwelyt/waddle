use avr_progmem::progmem;
use avr_progmem::wrapper::ProgMem;

use k::norde::se;
use Key::{Dead, KeyCode, LayerCh};

use crate::keycode::k;
use crate::keycode::k::layer;
use crate::layout::Key::{Function, LayerMo};
use crate::position::position::Position;
use crate::state::State;

#[derive(Copy, Clone)]
pub enum Key {
    KeyCode(u8),
    Function(fn(&mut State)),
    LayerMo(u8),
    LayerCh(u8),
    Dead,
}

pub const ROWS: usize = 4;
pub const COLS: usize = 12;
pub const BUTTONS: usize = ROWS * COLS;
pub const NUM_CHUNKS: usize = BUTTONS / 6;
pub const LAYERS: usize = 4;
pub const LEDS: usize = 2;
progmem! {
    pub static progmem MATRIX: [[[Key; COLS]; ROWS]; LAYERS] = [
        [
            [KeyCode(k::TAB), KeyCode(k::Q), KeyCode(k::W), KeyCode(k::E), KeyCode(k::R), KeyCode(k::T), KeyCode(k::Y), KeyCode(k::U), KeyCode(k::I), KeyCode(k::O), KeyCode(k::P), KeyCode(se::Å), ],
            [KeyCode(k::ESC), KeyCode(k::A), KeyCode(k::S), KeyCode(k::D), KeyCode(k::F), KeyCode(k::G), KeyCode(k::H), KeyCode(k::J), KeyCode(k::K), KeyCode(k::L), KeyCode(se::Ö), KeyCode(se::Ä), ],
            [KeyCode(k::L_SHFT), KeyCode(k::Z), KeyCode(k::X), KeyCode(k::C), KeyCode(k::V), KeyCode(k::B), KeyCode(k::N), KeyCode(k::M), KeyCode(k::COMMA), KeyCode(k::DOT), KeyCode(se::DASH), KeyCode(k::R_SHFT), ],
            [KeyCode(k::L_CTRL), KeyCode(k::L_SUPR), KeyCode(k::BS_N_PIPE), KeyCode(k::L_ALT), LayerMo(1), KeyCode(k::SPACE), KeyCode(k::RETURN), LayerMo(2), KeyCode(k::R_ALT), KeyCode(k::MENU), KeyCode(k::R_SUPR), KeyCode(k::R_CTRL), ],
        ],
        [
            [KeyCode(k::K1), KeyCode(k::K2), KeyCode(k::K3), KeyCode(k::K4), KeyCode(k::K5), KeyCode(k::K6), KeyCode(k::K7), KeyCode(k::K8), KeyCode(k::K9), KeyCode(k::K0), KeyCode(k::OBRAKET), KeyCode(k::CBRAKET), ],
            [KeyCode(k::ESC), Dead,Dead,Dead,Dead,Dead,Dead,Dead,Dead,Dead,Dead,Dead,],
            [KeyCode(k::L_SHFT), Dead,Dead,Dead,Dead,Dead,Dead,Dead,Dead,Dead,Dead,KeyCode(k::R_SHFT),],
            [KeyCode(k::L_CTRL), KeyCode(k::L_SUPR), Dead, KeyCode(k::L_ALT), Dead, KeyCode(k::SPACE), KeyCode(k::RETURN), LayerMo(2), KeyCode(k::R_ALT), KeyCode(k::MENU), KeyCode(k::R_SUPR), KeyCode(k::R_CTRL), ],
        ],
        [
            [KeyCode(k::F1), KeyCode(k::F2), KeyCode(k::F3), KeyCode(k::F4), KeyCode(k::F5), KeyCode(k::F6), KeyCode(k::F7), KeyCode(k::F8), KeyCode(k::F9), KeyCode(k::F10), KeyCode(k::F11), KeyCode(k::F12), ],
            [KeyCode(k::ESC), Dead,Dead,Dead,Dead,Dead,Dead,Dead,Dead,Dead,Dead,Dead,],
            [KeyCode(k::L_SHFT), Dead,Dead,Dead,Dead,Dead,Dead,Dead,Dead,Dead,Dead,KeyCode(k::R_SHFT),],
            [KeyCode(k::L_CTRL), KeyCode(k::L_SUPR), Dead, KeyCode(k::L_ALT), LayerMo(1), KeyCode(k::SPACE), KeyCode(k::RETURN), Dead, KeyCode(k::R_ALT), KeyCode(k::MENU), KeyCode(k::R_SUPR), KeyCode(k::R_CTRL), ],
        ],
        [
            [Function(|state| state.toggle_led(1)),               Function(|s|s.toggle_led(2)),               Function(|s| s.toggle_led(3)), Dead,              Dead, Dead,              Dead,               Dead, Dead,              Dead,             Dead,               Dead,               ],
            [KeyCode(k::ESC),    Dead,               Dead, Dead,              Dead, Dead,              Dead,               Dead, Dead,              Dead,             Dead,               Dead,               ],
            [KeyCode(k::L_SHFT), Dead,               Dead, Dead,              Dead, Dead,              Dead,               Dead, Dead,              Dead,             Dead,               KeyCode(k::R_SHFT), ],
            [KeyCode(k::L_CTRL), KeyCode(k::L_SUPR), Dead, KeyCode(k::L_ALT), Dead, KeyCode(k::SPACE), KeyCode(k::RETURN), Dead, KeyCode(k::R_ALT), KeyCode(k::MENU), KeyCode(k::R_SUPR), KeyCode(k::R_CTRL), ],
        ],
    ];
}

pub static LAYOUT: Layout = Layout { matrix: MATRIX };

pub struct Layout {
    matrix: ProgMem<[[[Key; COLS]; ROWS]; LAYERS]>,
}


impl Layout {
    pub fn new() -> Self {
        Self { matrix: MATRIX }
    }
    pub fn get_layer_mod(&self, position: &Position) -> u8 {
        match self.matrix.at(0)
            .at(position.row() as usize)
            .load_at(position.col() as usize) {
            KeyCode(_) => { 0 }
            Function(_) => { 0 }
            LayerMo(l) => { l }
            LayerCh(_) => { 0 }
            Dead => { 0 }
        }
    }

    pub fn get_keycode(&self, layer: u8, position: &Position) -> Option<u8> {
        match self.matrix.at(layer as usize)
            .at(position.row() as usize)
            .load_at(position.col() as usize) {
            KeyCode(kc) => { Some(kc) }
            Function(_) => { None }
            LayerMo(_) => { None }
            LayerCh(_) => { None }
            Dead => { None }
        }
    }

    pub fn get_mod(&self, layer: u8, position: &Position) -> Option<u8> {
        self.get_keycode(layer, position).filter(k::is_mod).map(k::to_mod_bitfield)
    }

    pub fn get_non_mod(&self, layer: u8, position: &Position) -> Option<u8> {
        self.get_keycode(layer, position).filter(|u| !k::is_mod(u))
    }


    pub fn apply_functions(&self, state: &State) -> State {
        let layer: u8 = state.pressed()
            .iter()
            .map(|p| LAYOUT.get_layer_mod(p))
            .sum();

        let mut s = state.clone();
        state.pressed()
            .iter()
            .for_each(|p| {
                match self.matrix.at(layer as usize)
                    .at(p.row() as usize)
                    .load_at(p.col() as usize) {
                    Function(f) => f(&mut s),
                    _ => {}
                }
            });
        s
    }
}
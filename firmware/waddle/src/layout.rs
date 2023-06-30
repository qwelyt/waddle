use avr_progmem::progmem;
use avr_progmem::wrapper::ProgMem;

use k::norde::se;
use Key::{Dead, Function, KeyCode, LayerMo, PassThrough};
use KeyType::{Instant, OnHold};

use crate::keycode::k;
use crate::keycode::k::layer;
use crate::position::position::Position;
use crate::state::State;

#[derive(Copy, Clone)]
pub enum KeyType {
    Instant(Key),
    OnHold(Key, u8, Key), // Press key, wait time, hold key
}

#[derive(Copy, Clone)]
pub enum Key {
    KeyCode(u8),
    Function(fn(&mut State)),
    LayerMo(u8),
    PassThrough(u8),
    Dead,
}

pub const ROWS: usize = 4;
pub const COLS: usize = 12;
pub const BUTTONS: usize = ROWS * COLS;
pub const NUM_CHUNKS: usize = BUTTONS / 6;
pub const LAYERS: usize = 4;
pub const LEDS: usize = 3;
// @formatter:off
progmem! {
    pub static progmem MATRIX: [[[KeyType; COLS]; ROWS]; LAYERS] = [
        [
            [Instant(KeyCode(k::TAB)),    Instant(KeyCode(k::Q)),      Instant(KeyCode(k::W)),         Instant(KeyCode(k::E)),     Instant(KeyCode(k::R)),  Instant(KeyCode(k::T)),      Instant(KeyCode(k::Y)),       Instant(KeyCode(k::U)),       Instant(KeyCode(k::I)),       Instant(KeyCode(k::O)),       Instant(KeyCode(k::P)),         Instant(KeyCode(se::Å)),        ],
            [Instant(KeyCode(k::ESC)),    Instant(KeyCode(k::A)),      Instant(KeyCode(k::S)),         Instant(KeyCode(k::D)),     Instant(KeyCode(k::F)),  Instant(KeyCode(k::G)),      Instant(KeyCode(k::H)),       Instant(KeyCode(k::J)),       Instant(KeyCode(k::K)),       Instant(KeyCode(k::L)),       Instant(KeyCode(se::Ö)),        Instant(KeyCode(se::Ä)),        ],
            [Instant(KeyCode(k::L_SHFT)), OnHold(KeyCode(k::Z), 100, KeyCode(k::L_SHFT)),      Instant(KeyCode(k::X)),         Instant(KeyCode(k::C)),     Instant(KeyCode(k::V)),  Instant(KeyCode(k::B)),      Instant(KeyCode(k::N)),       Instant(KeyCode(k::M)),       Instant(KeyCode(k::COMMA)),   Instant(KeyCode(k::DOT)),     Instant(KeyCode(se::DASH)),     Instant(KeyCode(k::R_SHFT)),    ],
            [Instant(KeyCode(k::L_CTRL)), Instant(KeyCode(k::L_SUPR)), Instant(KeyCode(k::BS_N_PIPE)), Instant(KeyCode(k::L_ALT)), Instant(LayerMo(1)),     Instant(KeyCode(k::SPACE)),  Instant(KeyCode(k::RETURN)),  Instant(LayerMo(2)),          Instant(KeyCode(k::R_ALT)),   Instant(KeyCode(k::MENU)),    Instant(KeyCode(k::R_SUPR)),    Instant(KeyCode(k::R_CTRL)),    ],
        ],
        [
            [Instant(KeyCode(k::K1)),     Instant(KeyCode(k::K2)),     Instant(KeyCode(k::K3)),        Instant(KeyCode(k::K4)),    Instant(KeyCode(k::K5)), Instant(KeyCode(k::K6)),     Instant(KeyCode(k::K7)),      Instant(KeyCode(k::K8)),      Instant(KeyCode(k::K9)),      Instant(KeyCode(k::K0)),      Instant(KeyCode(k::OBRAKET)),   Instant(KeyCode(k::CBRAKET)),   ],
            [Instant(PassThrough(1)),     Instant(PassThrough(1)),     Instant(PassThrough(1)),        Instant(PassThrough(1)),    Instant(PassThrough(1)), Instant(PassThrough(1)),     Instant(KeyCode(k::ARROW_L)), Instant(KeyCode(k::ARROW_D)), Instant(KeyCode(k::ARROW_U)), Instant(KeyCode(k::ARROW_R)), Instant(KeyCode(k::TILDE)),     Instant(KeyCode(k::EQUAL)),     ],
            [Instant(PassThrough(1)),     Instant(PassThrough(1)),     Instant(PassThrough(1)),        Instant(PassThrough(1)),    Instant(PassThrough(1)), Instant(PassThrough(1)),     Instant(PassThrough(1)),      Instant(PassThrough(1)),      Instant(PassThrough(1)),      Instant(PassThrough(1)),      Instant(PassThrough(1)),        Instant(KeyCode(k::BACKSPACE)), ],
            [Instant(PassThrough(1)),     Instant(PassThrough(1)),     Instant(KeyCode(k::GACC)),      Instant(PassThrough(1)),    Instant(PassThrough(1)), Instant(PassThrough(1)),     Instant(PassThrough(1)),      Instant(LayerMo(2)),          Instant(PassThrough(1)),      Instant(PassThrough(1)),      Instant(PassThrough(1)),        Instant(PassThrough(1)),        ],
        ],
        [
            [Instant(KeyCode(k::F1)),     Instant(KeyCode(k::F2)),     Instant(KeyCode(k::F3)),        Instant(KeyCode(k::F4)),    Instant(KeyCode(k::F5)), Instant(KeyCode(k::F6)),     Instant(KeyCode(k::F7)),      Instant(KeyCode(k::F8)),      Instant(KeyCode(k::F9)),      Instant(KeyCode(k::F10)),     Instant(KeyCode(k::F11)),       Instant(KeyCode(k::F12)),       ],
            [Instant(PassThrough(1)),     Instant(PassThrough(1)),     Instant(PassThrough(1)),        Instant(PassThrough(1)),    Instant(PassThrough(1)), Instant(KeyCode(k::INSERT)), Instant(KeyCode(k::HOME)),    Instant(KeyCode(k::PGDWN)),   Instant(KeyCode(k::PGUP)),    Instant(KeyCode(k::END)),     Instant(KeyCode(k::PRNT_SCRN)), Instant(KeyCode(k::DASH)),      ],
            [Instant(PassThrough(1)),     Instant(PassThrough(1)),     Instant(PassThrough(1)),        Instant(PassThrough(1)),    Instant(PassThrough(1)), Instant(PassThrough(1)),     Instant(PassThrough(1)),      Instant(PassThrough(1)),      Instant(PassThrough(1)),      Instant(PassThrough(1)),      Instant(PassThrough(1)),        Instant(KeyCode(k::DELETE)),    ],
            [Instant(PassThrough(1)),     Instant(PassThrough(1)),     Instant(PassThrough(1)),        Instant(PassThrough(1)),    Instant(LayerMo(1)),     Instant(PassThrough(1)),     Instant(PassThrough(1)),      Instant(PassThrough(1)),      Instant(PassThrough(1)),      Instant(PassThrough(1)),      Instant(PassThrough(1)),        Instant(PassThrough(1)),        ],
        ],
        [
            [Instant(Function(|state| state.toggle_led(0))), Instant(Function(|s|s.toggle_led(1))), Instant(Function(|s| s.toggle_led(2))), Instant(PassThrough(1)), Instant(PassThrough(1)), Instant(PassThrough(1)), Instant(PassThrough(1)), Instant(PassThrough(1)), Instant(PassThrough(1)), Instant(PassThrough(1)),Instant(PassThrough(1)), Instant(PassThrough(1)),],
            [Instant(PassThrough(1)),     Instant(PassThrough(1)),     Instant(PassThrough(1)),        Instant(PassThrough(1)),    Instant(PassThrough(1)), Instant(PassThrough(1)),    Instant(PassThrough(1)),      Instant(PassThrough(1)),      Instant(PassThrough(1)),      Instant(PassThrough(1)),       Instant(PassThrough(1)),        Instant(PassThrough(1)),        ],
            [Instant(PassThrough(1)),     Instant(PassThrough(1)),     Instant(PassThrough(1)),        Instant(PassThrough(1)),    Instant(PassThrough(1)), Instant(PassThrough(1)),    Instant(PassThrough(1)),      Instant(PassThrough(1)),      Instant(PassThrough(1)),      Instant(PassThrough(1)),       Instant(PassThrough(1)),        Instant(KeyCode(k::R_SHFT)),    ],
            [Instant(PassThrough(1)),     Instant(PassThrough(1)),     Instant(PassThrough(1)),        Instant(PassThrough(1)),    Instant(PassThrough(1)), Instant(PassThrough(1)),    Instant(PassThrough(1)),      Instant(PassThrough(1)),      Instant(PassThrough(1)),      Instant(PassThrough(1)),       Instant(PassThrough(1)),        Instant(PassThrough(1)),        ],
        ],
    ];
}
// @formatter:on

pub static LAYOUT: Layout = Layout { matrix: MATRIX };

pub struct Layout {
    matrix: ProgMem<[[[KeyType; COLS]; ROWS]; LAYERS]>,
}


impl Layout {
    pub fn new() -> Self {
        Self { matrix: MATRIX }
    }

    pub fn get_key(&self, layer: u8, position: &Position) -> KeyType {
        self.matrix.at(layer as usize).at(position.row() as usize).at(position.col() as usize).load()
    }
}
use alloc::vec::Vec;

use crate::Key::Key;

pub const LAYERS: u8 = 4; // 0 (normal), 1,2, 1+2==3
pub const COLS: u8 = 12;
pub const ROWS: u8 = 4;

pub struct Layout {
    mapping: [[[Key; COLS as usize]; ROWS as usize]; LAYERS as usize],
}

#[rustfmt::skip]
pub fn layout() -> Layout {
    Layout {
        mapping: [
            [ 
                [   Key::TAB   , Key::Q     , Key::W       , Key::E    , Key::R      , Key::T      , Key::Y      , Key::U      , Key::I      , Key::O      , Key::P        , Key::OBRAKET   ]
                , [ Key::ESC   , Key::A     , Key::S       , Key::D    , Key::F      , Key::G      , Key::H      , Key::J      , Key::K      , Key::L      , Key::COLON    , Key::QUOTE     ]
                , [ Key::LShft , Key::Z     , Key::X       , Key::C    , Key::V      , Key::B      , Key::N      , Key::M      , Key::COMMA  , Key::DOT    , Key::SLASH    , Key::BACKSPACE ]
                , [ Key::LCtrl , Key::LSupr , Key::BsNPipe , Key::LAlt , Key::Layer1 , Key::SPACE  , Key::RETURN , Key::Layer2 , Key::RAlt   , Key::MENU   , Key::RSupr     , Key::RCtrl     ]
            ]
            , [
                [   Key::K1    , Key::K2    , Key::K3      , Key::K4   , Key::K5     , Key::K6     , Key::K7     , Key::K8     , Key::K9     , Key::K0     , Key::OBRAKET  , Key::CBRAKET   ]
                , [ Key::ESC   , Key::NONE  , Key::NONE    , Key::NONE , Key::NONE   , Key::NONE   , Key::ArrowL , Key::ArrowD , Key::ArrowU , Key::ArrowR , Key::TILDE    , Key::EQUAL     ]
                , [ Key::LShft , Key::NONE  , Key::NONE    , Key::NONE , Key::NONE   , Key::NONE   , Key::NONE   , Key::NONE   , Key::NONE   , Key::NONE   , Key::NONE     , Key::DELETE    ]
                , [ Key::LCtrl , Key::LSupr , Key::BsNPipe , Key::LAlt , Key::Layer1 , Key::SPACE  , Key::RETURN , Key::Layer2 , Key::RAlt   , Key::MENU   , Key::RSupr    , Key::RCtrl     ]
            ]
            , [ // Layer2
                [   Key::F1    , Key::F2    , Key::F3      , Key::F4   , Key::F5     , Key::F6     , Key::F7     , Key::F8     , Key::F9     , Key::F10    , Key::F11      , Key::F12       ]
                , [ Key::ESC   , Key::NONE  , Key::NONE    , Key::NONE , Key::NONE   , Key::INSERT , Key::HOME   , Key::PgDwn, Key::PgUp, Key::END    , Key::PrntScrn , Key::DASH      ]
                , [ Key::LShft , Key::NONE  , Key::NONE    , Key::NONE , Key::NONE   , Key::NONE   , Key::NONE   , Key::NONE   , Key::NONE   , Key::NONE   , Key::NONE     , Key::DELETE    ]
                , [ Key::LCtrl , Key::LSupr , Key::BsNPipe , Key::LAlt , Key::Layer1 , Key::SPACE  , Key::RETURN , Key::Layer2 , Key::RAlt   , Key::MENU   , Key::RSupr    , Key::RCtrl     ]
            ]
            , [ // Layer3
                [   Key::NONE  , Key::NONE  , Key::NONE    , Key::NONE , Key::NONE   , Key::NONE   , Key::NONE   , Key::NONE   , Key::NONE   , Key::NONE   , Key::NONE     , Key::NONE      ]
                , [ Key::ESC   , Key::NONE  , Key::NONE    , Key::NONE , Key::NONE   , Key::NONE   , Key::NONE   , Key::NONE   , Key::NONE   , Key::NONE   , Key::NONE     , Key::NONE      ]
                , [ Key::LShft , Key::NONE  , Key::NONE    , Key::NONE , Key::NONE   , Key::NONE   , Key::NONE   , Key::NONE   , Key::NONE   , Key::NONE   , Key::NONE     , Key::NONE      ]
                , [ Key::LCtrl , Key::LSupr , Key::BsNPipe , Key::LAlt , Key::Layer1 , Key::SPACE  , Key::RETURN , Key::Layer2 , Key::RAlt   , Key::MENU   , Key::RSupr    , Key::RCtrl     ]
            ]
        ]
    }
}

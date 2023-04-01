pub mod k {
    pub const NONE: u8 = 0x00;
    pub const A: u8 = 0x04;
    pub const B: u8 = 0x05;
    pub const C: u8 = 0x06;
    pub const D: u8 = 0x07;
    pub const E: u8 = 0x08;
    pub const F: u8 = 0x09;
    pub const G: u8 = 0x0A;
    pub const H: u8 = 0x0B;
    pub const I: u8 = 0x0C;
    pub const J: u8 = 0x0D;
    pub const K: u8 = 0x0E;
    pub const L: u8 = 0x0F;
    pub const M: u8 = 0x10;
    pub const N: u8 = 0x11;
    pub const O: u8 = 0x12;
    pub const P: u8 = 0x13;
    pub const Q: u8 = 0x14;
    pub const R: u8 = 0x15;
    pub const S: u8 = 0x16;
    pub const T: u8 = 0x17;
    pub const U: u8 = 0x18;
    pub const V: u8 = 0x19;
    pub const W: u8 = 0x1A;
    pub const X: u8 = 0x1B;
    pub const Y: u8 = 0x1C;
    pub const Z: u8 = 0x1D;

    pub const K1: u8 = 0x1E;
    pub const K2: u8 = 0x1F;
    pub const K3: u8 = 0x20;
    pub const K4: u8 = 0x21;
    pub const K5: u8 = 0x22;
    pub const K6: u8 = 0x23;
    pub const K7: u8 = 0x24;
    pub const K8: u8 = 0x25;
    pub const K9: u8 = 0x26;
    pub const K0: u8 = 0x27;

    pub const RETURN: u8 = 0x28;
    pub const ESC: u8 = 0x29;
    pub const BACKSPACE: u8 = 0x2A;
    pub const TAB: u8 = 0x2B;
    pub const SPACE: u8 = 0x2C;
    pub const DASH: u8 = 0x2D;
    pub const EQUAL: u8 = 0x2E;
    pub const OBRAKET: u8 = 0x2F;
    pub const CBRAKET: u8 = 0x30;
    pub const BSLASH: u8 = 0x31;
    pub const TILDE: u8 = 0x32;
    pub const COLON: u8 = 0x33;
    pub const QUOTE: u8 = 0x34;
    pub const GACC: u8 = 0x35;
    pub const COMMA: u8 = 0x36;
    pub const DOT: u8 = 0x37;
    pub const SLASH: u8 = 0x38;
    pub const CAPSLOCK: u8 = 0x39;

    pub const F1: u8 = 0x3A;
    pub const F2: u8 = 0x3B;
    pub const F3: u8 = 0x3C;
    pub const F4: u8 = 0x3D;
    pub const F5: u8 = 0x3E;
    pub const F6: u8 = 0x3F;
    pub const F7: u8 = 0x40;
    pub const F8: u8 = 0x41;
    pub const F9: u8 = 0x42;
    pub const F10: u8 = 0x43;
    pub const F11: u8 = 0x44;
    pub const F12: u8 = 0x45;

    pub const PRNT_SCRN: u8 = 0x46;
    pub const SCRL_LCK: u8 = 0x47;
    pub const PAUSE: u8 = 0x48;
    pub const INSERT: u8 = 0x49;
    pub const HOME: u8 = 0x4A;
    pub const PGUP: u8 = 0x4B;
    pub const DELETE: u8 = 0x4C;
    pub const END: u8 = 0x4D;
    pub const PGDWN: u8 = 0x4E;
    pub const ARROW_R: u8 = 0x4F;
    pub const ARROW_L: u8 = 0x50;
    pub const ARROW_D: u8 = 0x51;
    pub const ARROW_U: u8 = 0x52;

    pub const NUM_LCK: u8 = 0x53;
    pub const N_DIV: u8 = 0x54;
    pub const N_MUL: u8 = 0x55;
    pub const N_SUB: u8 = 0x56;
    pub const N_ADD: u8 = 0x57;
    pub const N_ENTER: u8 = 0x58;
    pub const N1: u8 = 0x59;
    pub const N2: u8 = 0x5A;
    pub const N3: u8 = 0x5B;
    pub const N4: u8 = 0x5C;
    pub const N5: u8 = 0x5D;
    pub const N6: u8 = 0x5E;
    pub const N7: u8 = 0x5F;
    pub const N8: u8 = 0x60;
    pub const N9: u8 = 0x61;
    pub const N0: u8 = 0x62;
    pub const NDOT: u8 = 0x63;

    pub const BS_N_PIPE: u8 = 0x64;
    pub const MENU: u8 = 0x65;


    // Special
    pub const L_CTRL: u8 = 0xE0;
    pub const L_SHFT: u8 = 0xE1;
    pub const L_ALT: u8 = 0xE2;
    pub const L_SUPR: u8 = 0xE3;
    pub const R_CTRL: u8 = 0xE4;
    pub const R_SHFT: u8 = 0xE5;
    pub const R_ALT: u8 = 0xE6;
    pub const R_SUPR: u8 = 0xE7;


    pub mod layer {
        pub const LOWER: u8 = 0x00;
        pub const RAISE: u8 = 0x00;
    }

    pub mod norde {
        pub mod se {
            use crate::keycode::k::{COLON, OBRAKET, QUOTE, SLASH};

            pub const Å: u8 = OBRAKET;
            pub const Ö: u8 = COLON;
            pub const Ä: u8 = QUOTE;
            pub const DASH: u8 = SLASH;
        }
    }
}
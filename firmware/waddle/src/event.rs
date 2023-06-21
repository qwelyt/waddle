#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Event {
    KeyCode(u8),
    // Adding another enum here breaks *everything*. Does not matter if it's used or not, this
    // Breaks the USB setup. I can add enums in `Key` without problem, but here it breaks.
    // Function
}


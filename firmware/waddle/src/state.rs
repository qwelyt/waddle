#[derive(Copy, Clone, Eq, PartialEq)]
pub struct State {
    layer: u8,

}


impl State {
    pub fn new() -> Self {
        Self {
            layer: 0
        }
    }
    pub fn layer_mo(&mut self, layer: u8) {
        self.layer += layer
    }
}

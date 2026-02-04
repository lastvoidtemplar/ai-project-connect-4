pub const WIDTH: usize = 7;
pub const HEIGHT: usize = 6;

pub trait Position {
    fn can_play(&self, colm:usize) -> bool;
    fn play(&mut self, colm:usize);
    fn reverse_play(&mut self, colm:usize);
    fn is_winning(&self, colm:usize) -> bool;
    fn played_moves(&self) -> usize;
}

pub fn load_starting_position<P: Position>(encoded_position: &str, position: &mut P){
    for ch in encoded_position.as_bytes() {
        let colm = (ch - '1' as u8) as usize;
        if colm >= WIDTH || !position.can_play(colm) || position.is_winning(colm) {
            panic!("invalid starting position");
        }
        position.play(colm);
    }
}

pub mod array_position;
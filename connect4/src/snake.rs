use canvas::Canvas;
use stdweb::unstable::TryInto;
use direction::Direction;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Block(u32, u32);

#[derive(Debug)]
pub struct Snake {
    head: Block,
    tail: Vec<Block>,
    food: Block,
    height: u32,
    width: u32,
    direction: Option<Direction>,
    next_direction: Option<Direction>,
    last_direction: Direction,
}

impl Snake {

}

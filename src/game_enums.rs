/// Enumerates the possible SOS cell values
#[derive(Clone, PartialEq, Debug, Copy)]
pub enum Cell { Empty, S, O}

/// Enumerates the different game modes
#[derive(Clone, PartialEq, Debug)]
pub enum Mode { Classic, Simple }

/// Enumerates player turns
#[derive(PartialEq, Debug)]
pub enum Turn { Left, Right }

#[derive(PartialEq, Debug)]
pub enum State { LeftWin, RightWin, Draw, Playing, NotStarted }
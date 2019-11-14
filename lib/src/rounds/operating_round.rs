/// Represents an operating round
#[derive(Clone, Debug)]
pub struct OperatingRound {
    operating_rounds_left: u32,
}

impl OperatingRound {
    pub(crate) fn new(operating_rounds_left: u32) -> Self {
        OperatingRound {
            operating_rounds_left,
        }
    }
}

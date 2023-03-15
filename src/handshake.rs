pub struct Handshake;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Actions {
    Wink,
    DoubleBlink,
    CloseYourEyes,
    Jump,
}

impl Handshake {
    pub fn parse(value: u16) -> Option<Vec<Actions>> {
        let mut actions: Vec<Actions> = Vec::new();
        let binary = format!("{:b}", value);
        for (i, v) in binary.chars().rev().enumerate() {
            if v == '1' {
                match i {
                    0 => actions.push(Actions::Wink),
                    1 => actions.push(Actions::DoubleBlink),
                    2 => actions.push(Actions::CloseYourEyes),
                    3 => actions.push(Actions::Jump),
                    4 => actions.reverse(),
                    _ => {
                      return None;
                    },
                }
            }
        }
        Some(actions)
    }


}

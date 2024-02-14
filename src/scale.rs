pub struct Scale {
    tonic: u8,
    mode: Mode,
}

impl Scale {
    pub fn pitches(&self) -> [u8; 7] {
        self.mode.offsets().map(|x| x + self.tonic)
    }
}

pub enum Mode {
    Major,
    Minor,
}

impl Mode {
    pub fn offsets(&self) -> [u8; 7] {
        match self {
            Mode::Major => [0, 2, 4, 5, 7, 9, 11],
            Mode::Minor => [0, 2, 3, 5, 7, 8, 10],
        }
    }
}

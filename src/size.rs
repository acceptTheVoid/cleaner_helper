use std::ops::AddAssign;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Size(u64);

impl Size {
    pub const fn new(size: u64) -> Self {
        Self(size)
    }
}

impl AddAssign for Size {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl std::fmt::Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut len = self.0 as f64;
        let mut t = 0;
        while len > 1024. {
            len /= 1024.;
            t += 1;
        }

        let notation = match t {
            0 => "b",
            1 => "Kb",
            2 => "Mb", 
            3 => "Gb",
            4 => "Tb",
            _ => todo!("Add support for more notation types"),
        };

        write!(f, "{len:.2}{notation}")
    }
}

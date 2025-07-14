use enum_assoc::Assoc;
use std::collections::HashMap;

macro_rules! names {
    ($s:literal) => {
        ($s, concat!($s, "s"))
    };
}

#[derive(Assoc, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[func(const fn names(self) -> (&'static str, &'static str))]
#[func(pub const fn base_cost(self) -> f64)]
#[func(pub const fn base_cps(self) -> f64)]
pub enum Building {
    #[assoc(names = names!("Cursor"), base_cost = 15.0, base_cps = 0.1)]
    Cursor,
    #[assoc(names = names!("Grandma"), base_cost = 100.0, base_cps = 1.0)]
    Grandma,
    #[assoc(names = names!("Farm"), base_cost = 1000.0, base_cps = 10.0)]
    Farm,
}

impl Building {
    const ALL: [Self; 3] = [Self::Cursor, Self::Grandma, Self::Farm];

    pub fn index(i: usize) -> Option<Self> {
        Self::ALL.get(i).copied()
    }

    pub const fn name(self) -> &'static str {
        self.names().0
    }

    pub const fn name_plural(self) -> &'static str {
        self.names().1
    }

    pub const fn name_pluralized(self, count: usize) -> &'static str {
        if count == 1 {
            self.name()
        } else {
            self.name_plural()
        }
    }
}

#[derive(Debug)]
pub struct Buildings {
    counts: HashMap<Building, u32>,
}

impl Buildings {
    pub fn new() -> Self {
        Self {
            counts: HashMap::new(),
        }
    }

    pub fn buy(&mut self, building: Building) {
        self.counts
            .entry(building)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    pub fn cps(&self) -> f64 {
        self.iter()
            .map(|(building, count)| building.base_cps() * count as f64)
            .sum()
    }

    pub fn count(&self, building: Building) -> u32 {
        self.counts.get(&building).copied().unwrap_or_default()
    }

    pub fn iter(&self) -> impl Iterator<Item = (Building, u32)> {
        Building::ALL.iter().map(|&b| (b, self.count(b)))
    }
}

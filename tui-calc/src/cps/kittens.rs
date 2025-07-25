pub struct Cps<MilkFactors> {
    pub milk_percentage: u16,
    pub kitten_factors: MilkFactors,
}

impl<MilkFactors> Cps<MilkFactors>
where
    MilkFactors: Iterator<Item = f64>,
{
    pub fn calc(self) -> f64 {
        let milk_mult_ratio = self.milk_percentage as f64 / 100.0;

        self.kitten_factors
            .map(|f| 1.0 + milk_mult_ratio * f)
            .product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx_eq_trait::assert_approx_eq;

    #[test]
    fn from_wiki() {
        assert_approx_eq!(
            Cps {
                milk_percentage: 200,
                kitten_factors: [0.1].into_iter()
            }
            .calc(),
            1.2,
        );
        assert_approx_eq!(
            Cps {
                milk_percentage: 200,
                kitten_factors: [0.15].into_iter()
            }
            .calc(),
            1.3,
        );
        assert_approx_eq!(
            Cps {
                milk_percentage: 200,
                kitten_factors: [0.1, 0.15].into_iter()
            }
            .calc(),
            1.56,
        );
    }
}

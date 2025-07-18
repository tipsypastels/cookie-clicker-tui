mod grandma_co_tiered;
mod simple_tiered;

#[derive(Debug)]
pub struct Upgrade(UpgradeInner);

#[derive(Debug)]
enum UpgradeInner {
    SimpleTiered(simple_tiered::SimpleTieredUpgrade),
    GrandmaCoTiered(grandma_co_tiered::GrandmaCoTieredUpgrade),
}

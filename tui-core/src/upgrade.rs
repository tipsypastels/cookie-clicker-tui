mod grandma_co_tiered;
mod simple_tiered;

#[derive(Debug)]
pub struct Upgrade(Inner);

#[derive(Debug)]
enum Inner {
    SimpleTiered(simple_tiered::SimpleTieredUpgrade),
    GrandmaCoTiered(grandma_co_tiered::GrandmaCoTieredUpgrade),
}

use crate::upgrade::grandma_job_num_req_for_1p;

pub struct Cps<AddlCpsPerOwnedBuildingCounts> {
    pub building_no: u16,
    pub building_base_cps: f64,
    pub building_class: CpsClass,
    pub count: u16,
    pub tiered_upgrade_count: u16,
    pub addl_cps_per_owned_building_counts: AddlCpsPerOwnedBuildingCounts,
}

pub enum CpsClass {
    Cursor,
    Grandma {
        has_bingo_center_4x: bool,
        job_upgrade_count: u16,
    },
    Other {
        grandma_count: Option<u16>,
    },
}

impl<AddlCpsPerOwnedBuildingCounts> Cps<AddlCpsPerOwnedBuildingCounts>
where
    AddlCpsPerOwnedBuildingCounts: Iterator<Item = (u16, f64)>,
{
    pub fn calc(self) -> f64 {
        let Self {
            building_no,
            building_base_cps,
            building_class,
            count,
            tiered_upgrade_count,
            addl_cps_per_owned_building_counts,
        } = self;

        let cps = building_base_cps * count as f64 * 2.0f64.powi(tiered_upgrade_count as i32);

        let cps = match building_class {
            CpsClass::Cursor => cps,
            CpsClass::Grandma {
                has_bingo_center_4x,
                job_upgrade_count,
            } => {
                cps * if has_bingo_center_4x { 4.0 } else { 1.0 }
                    * 2.0f64.powi(job_upgrade_count as i32)
            }
            CpsClass::Other {
                grandma_count: Some(grandma_count),
            } => {
                let num_req_for_1p_increase = grandma_job_num_req_for_1p(building_no);

                if grandma_count > num_req_for_1p_increase {
                    let ratio = grandma_count / num_req_for_1p_increase;
                    let addl = ratio as f64 * 0.01;
                    cps + addl
                } else {
                    cps
                }
            }
            CpsClass::Other {
                grandma_count: None,
            } => cps,
        };

        let cps = cps
            + addl_cps_per_owned_building_counts
                .map(|(count, cps)| count as f64 * cps)
                .sum::<f64>();

        cps
    }
}

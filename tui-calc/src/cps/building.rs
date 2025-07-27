use crate::{thousand_fingers::ThousandFingers, upgrade::grandma_job_num_req_for_1p};

pub struct Cps {
    pub building_no: u16,
    pub building_base_cps: f64,
    pub building_class: CpsClass,
    pub count: u16,
    pub tiered_upgrade_count: u16,
}

pub enum CpsClass {
    Cursor {
        thousand_fingers: Option<ThousandFingers>,
    },
    Grandma {
        has_bingo_center: bool,
        has_one_mind: bool,
        has_communal_brainsweep: bool,
        elder_pact_portal_count: Option<u16>,
        job_upgrade_count: u16,
    },
    Other {
        grandma_count: Option<u16>,
    },
}

impl Cps {
    pub fn calc(self) -> f64 {
        let Self {
            building_no,
            building_base_cps,
            building_class,
            count,
            tiered_upgrade_count,
        } = self;

        let cps = building_base_cps * count as f64 * 2.0f64.powi(tiered_upgrade_count as i32);

        let cps = match building_class {
            CpsClass::Cursor {
                thousand_fingers: None,
            } => cps,
            CpsClass::Cursor {
                thousand_fingers: Some(thousand_fingers),
            } => cps + thousand_fingers.calc(),
            CpsClass::Grandma {
                has_bingo_center,
                has_one_mind,
                has_communal_brainsweep,
                elder_pact_portal_count,
                job_upgrade_count,
            } => {
                cps * if has_bingo_center { 4.0 } else { 1.0 }
                    * 2.0f64.powi(job_upgrade_count as i32)
                    + if has_one_mind {
                        0.02 * count as f64
                    } else {
                        0.0
                    }
                    + if has_communal_brainsweep {
                        0.02 * count as f64
                    } else {
                        0.0
                    }
                    + if let Some(portal_count) = elder_pact_portal_count {
                        0.05 * portal_count as f64
                    } else {
                        0.0
                    }
            }
            CpsClass::Other {
                grandma_count: None,
            } => cps,
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
        };

        cps
    }
}

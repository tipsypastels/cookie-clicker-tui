pub fn grandma_job_num_req_for_1p(building_no: u16) -> u16 {
    // the cursors and grandmas themselves cannot get a grandma job upgrade
    debug_assert!(building_no >= 2);

    // farm = #2, gains per 1 grandma
    // miner = #3, gains per 2 grandmas,
    // ...
    building_no - 1
}

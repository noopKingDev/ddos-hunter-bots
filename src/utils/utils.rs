use std::time::{SystemTime, UNIX_EPOCH};


fn generation_rand_number() -> u64 {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let seed = now.as_secs() ^ now.subsec_nanos() as u64;
    seed % 255
}
pub fn generation_ips() -> String {


        let octet1 = generation_rand_number();
        let octet2 = generation_rand_number();
        let octet3 = generation_rand_number();
        let octet4 = generation_rand_number();
        
        format!("{}.{}.{}.{}", octet1, octet2, octet3, octet4)


}
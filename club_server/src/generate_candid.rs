// ######################
// Below code is to generate candid file
// ######################
#[cfg(test)]
mod generate_candid {
    use crate::export_candid;
    #[test]
    fn write_candid_to_disk() {
        std::fs::write("club_server.did", export_candid()).unwrap();
    }
}

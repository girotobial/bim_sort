use bim_sort::models::Bim;

#[test]
#[should_panic]
fn throws_error_on_unknown_fields_in_root() {
    let path = std::path::PathBuf::from("./bim_with_root_error.bim");
    Bim::from_file(&path).unwrap();
}

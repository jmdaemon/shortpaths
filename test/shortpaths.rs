#[test]
fn test_shortpaths() {
    use shortpaths::shortpaths::{
        SPT,
        Shortpath,
        ShortpathsBuilder,
        FindKeyIndexMapExt,
        sort_shortpaths,
        export_shortpaths,
    };

    use std::path::PathBuf;

    // TODO Create more ergonomic api for this later
    // Wrap it together with the builder construct to reduce the noise
    let sp_paths = vec![
        Shortpath::new(SPT::AliasPath("d".to_owned(), PathBuf::from("$a/dddd")), None, None),
        Shortpath::new(SPT::AliasPath("c".to_owned(), PathBuf::from("$b/cccc")), None, None),
        Shortpath::new(SPT::AliasPath("b".to_owned(), PathBuf::from("$a/bbbb")), None, None),
        Shortpath::new(SPT::Path("a".to_owned(), PathBuf::from("aaaa")), None, None),
    ];
    println!("{:?}", sp_paths);

    let mut sp_builder = ShortpathsBuilder::new(sp_paths);

    let sp_im = sp_builder.build().unwrap();
    sp_im.iter().for_each(|p| println!("{:?}", p));

    // Test find_key
    let key = sp_im.find_key_for_value("$a/bbbb");
    println!("{:?}", key);
    assert_ne!(None, key, "Can find keys from &str values");

    let key = sp_im.find_key_for_value("$a/bbbb".to_string());
    println!("{:?}", key);
    assert_ne!(None, key, "Can find keys from String values");

    // Test sort_shortpaths
    println!("Sorted list of shortpaths");
    let sorted = sort_shortpaths(sp_im);
    sorted.iter().for_each(|p| println!("{:?}", p));

    // Test serialization
    let export_type = "bash";
    let output_file = None;
    export_shortpaths(&sorted, export_type, output_file);
}

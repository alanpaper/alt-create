use regex::Regex;

// 判断项目名称是否有效
pub fn is_valid_package_name(project_name: String) -> bool {
    let regex =
        Regex::new(r"/^(?:@[a-z\d\-*~][a-z\d\-*._~]*\/)?[a-z\d\-~][a-z\d\-._~]*$/").unwrap();
    regex.is_match(&project_name)
}

// 转换不符合标准的项目名称
pub fn to_valid_package_name(project_name: String) -> String {
    let mut ans = project_name.to_ascii_lowercase();
    ans = ans.trim().to_lowercase();
    let mut re = Regex::new(r"/\s+/g").unwrap();
    ans = re.replace(&ans, "-").to_string();
    re = Regex::new(r"/^[._]/").unwrap();
    ans = re.replace(&ans, "").to_string();
    re = Regex::new(r"/[^a-z\d\-~]+/g").unwrap();
    ans = re.replace(&ans, "-").to_string();
    ans
}

#[test]
fn test_to_valid_package_name() {
    assert_eq!(
        to_valid_package_name(String::from("Alt-pRoject")),
        "alt-project"
    )
}

#[test]
fn test_to_valid_package_name_1() {
    assert_eq!(
        to_valid_package_name(String::from("altProject")),
        "altproject"
    )
}

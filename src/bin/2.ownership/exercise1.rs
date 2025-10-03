/*
問題 1：文字列

2つの文字列を結合し、新しい文字列を作成する関数を作成してください。
関数名: concatenate_strings
*/

fn concatenate_strings(s1: String, s2: String) -> String {
    format!("{}{}", s1, s2)
}

#[test]
fn test_concatenate_strings() {
    // テストケース1: 2つの文字列を結合する
    let s1 = String::from("hello");
    let s2 = String::from(" world");
    let result = concatenate_strings(s1, s2);
    assert_eq!(result, "hello world");
}

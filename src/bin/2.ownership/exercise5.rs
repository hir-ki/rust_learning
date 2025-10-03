/*
問題 5：参照とライフタイムの応用

print_longest という関数を作成します。
この関数は、2 つの文字列の参照を受け取り、より長い方の文字列を出力します。

関数の引数に渡される文字列のライフタイムを考慮し、コンパイルエラーが出ないようにします。
引数で渡された文字列のいずれかが変更された場合、関数の動作に影響が出ないようにします。
*/

// fn print_longest<'a>(s1: &'a str, s2: &'a str) {
//     if s1.len() == s2.len() {
//         println!("{} {} are same length", &s1, &s2);
//     } else if s1.len() > s2.len() {
//         println!("{} is longer", &s1);
//     } else {
//         println!("{} is longer", &s2);
//     }
// }

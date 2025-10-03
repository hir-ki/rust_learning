/*
問題 3：複数の所有権を持つ構造体
Book という構造体を定義します。この構造体は、title (文字列) と author (文字列) を持つものとします。
create_book という関数を作成し、title と author を引数として受け取り、Book 構造体のインスタンスを返します。
print_book_info という関数を作成し、Book 構造体の参照を受け取り、その情報をStringで返します。
main 関数内で、create_book を呼び出し、結果を別の変数に格納します。
Book 構造体の情報を print_book_info 関数を使って出力します。
*/

struct Book {
    title: String,
    author: String,
}

fn create_book(title: String, author: String) -> Book {
    Book { title, author }
}

fn print_book_info(book: Book) -> String {
    format!("タイトル: {}, 著者: {}", book.title, book.author)
}

fn main(title: String, author: String) -> Book {
    let book = create_book(title, author);
    book
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_print_book() {
        // 異なるタイトルと著者を持つBookを作成
        let book1 = main("Rust入門".to_string(), "太郎".to_string());
        let book2 = main("アルゴリズム入門".to_string(), "次郎".to_string());

        // 出力結果をアサート（具体的なアサートは、print_book_info関数の出力形式によって変更）
        assert_eq!(
            print_book_info(book1),
            "タイトル: Rust入門, 著者: 太郎".to_string()
        );
        assert_eq!(
            print_book_info(book2),
            "タイトル: アルゴリズム入門, 著者: 次郎".to_string()
        );
    }
}

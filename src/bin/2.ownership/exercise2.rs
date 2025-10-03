/*
問題 2：構造体と所有権の複雑な関係

Book 構造体と Library 構造体を作成します。
Book 構造体には、title と author というフィールドがあり、それぞれ String 型です。
Library 構造体には、books というフィールドがあり、これは Vec<Book>型です。
Library 構造体に対して、add_book メソッドを作成し、新しい Book 構造体を books ベクターに追加できるようにします。

add_book メソッドは、Book 構造体の所有権を受け取り、Library 構造体の books ベクターに追加します。
main 関数内で、複数の Book 構造体を作成し、Library 構造体に追加する処理を実装します。
解答のヒント:
Vec は、要素の所有権を取得します。
add_book メソッドは、Book 構造体の所有権を移動させる必要があります。
*/
struct Book {
    title: String,
    author: String,
}
struct Library {
    books: Vec<Book>,
}

impl Library {
    fn new() -> Self {
        Library { books: Vec::new() }
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_book() {
        let mut library = Library::new();

        let book = Book {
            title: String::from("テスト用書籍"),
            author: String::from("テスト太郎"),
        };

        library.add_book(book);

        assert_eq!(library.books.len(), 1);
        assert_eq!(library.books[0].title, "テスト用書籍");
        assert_eq!(library.books[0].author, "テスト太郎");
    }
}

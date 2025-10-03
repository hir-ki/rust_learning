/*
問題1: 図形の面積計算

Shape トレイトを定義し、area() メソッドを定義します。
Rectangle, Circle 構造体を定義し、Shape トレイトを実装します。
各構造体に、面積を計算する area() メソッドを実装します。
RectangleとCircleの面積を計算して表示させます。
*/

trait Shape {
    fn area(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}

fn main() {
    let rec = Rectangle {
        width: 2.0,
        height: 5.0,
    };
    let cir = Circle { radius: 3.0 };
    println!("Rectangle area is :{}", rec.area());
    println!("Circle area is :{}", cir.area());
}

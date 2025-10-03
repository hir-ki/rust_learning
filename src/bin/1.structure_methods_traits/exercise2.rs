/*

問題 2: 図形の面積計算と周囲の長さ計算

1.Shape トレイトに、area() メソッドに加えて、perimeter() メソッド（周囲の長さを求める）を追加します。
2.Rectangle, Circle 構造体に加え、Triangle 構造体を定義します。
3.各構造体に、それぞれの図形の面積と周囲の長さを計算するメソッドを実装します。
4.Triangle 構造体には、3 辺の長さを保持するフィールドと、三角形の成立条件を満たすためのチェックを行うコンストラクタを実装します。
*/

use std::f64::consts::PI;

trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn perimeter(&self) -> f64 {
        self.width * 2.0 + self.height * 2.0
    }
}
struct Circle {
    radius: f64,
}
impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * PI
    }
    fn perimeter(&self) -> f64 {
        self.radius * 2.0 * PI
    }
}

struct Triangle {
    edge_a: f64,
    edge_b: f64,
    edge_c: f64,
}
impl Shape for Triangle {
    // 3辺から面積を求めるヘロンの公式を利用
    fn area(&self) -> f64 {
        let s = (self.edge_a + self.edge_b + self.edge_c) / 2.0;
        (s * (s - self.edge_a) * (s - self.edge_b) * (s - self.edge_c)).sqrt()
    }
    fn perimeter(&self) -> f64 {
        self.edge_a + self.edge_b + self.edge_c
    }
}

impl Triangle {
    // 三角形が成立する条件：1辺の長さが、他の2辺の和よりも小さい 状態(a < b+c)
    fn new(edge_a: f64, edge_b: f64, edge_c: f64) -> Result<Triangle, String> {
        if edge_a >= edge_b + edge_c || edge_b >= edge_a + edge_c || edge_c >= edge_a + edge_b {
            return Err("The triangle inequality theorem is not satisfied".to_string());
        }
        Ok(Triangle {
            edge_a,
            edge_b,
            edge_c,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::EPSILON;

    #[test]
    fn test_rectangle() {
        let rect = Rectangle {
            width: 3.0,
            height: 4.0,
        };
        assert!((rect.area() - 12.0).abs() < EPSILON);
        assert!((rect.perimeter() - 14.0).abs() < EPSILON);
    }

    #[test]
    fn test_circle() {
        let circle = Circle { radius: 1.0 };
        assert!((circle.area() - PI).abs() < EPSILON);
        assert!((circle.perimeter() - 2.0 * PI).abs() < EPSILON);
    }

    #[test]
    fn test_triangle_creation() {
        // Valid triangle
        let triangle = Triangle::new(3.0, 4.0, 5.0);
        assert!(triangle.is_ok());

        // Invalid triangle - violates triangle inequality
        let invalid_triangle = Triangle::new(1.0, 1.0, 3.0);
        assert!(invalid_triangle.is_err());

        #[test]
        fn test_triangle_calculations() {
            let triangle = Triangle::new(3.0, 4.0, 5.0).unwrap();
            // Area should be 6.0 (using Heron's formula)
            assert!((triangle.area() - 6.0).abs() < EPSILON);
            // Perimeter should be 12.0
            assert!((triangle.perimeter() - 12.0).abs() < EPSILON);
        }
    }
}

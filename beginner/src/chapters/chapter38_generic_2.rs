pub fn run() {
    let p1 = Point { x: 2, y: 3 };
    let _p3 = Point { x: 2.4, y: 3.5 };
    println!("p1=({},{})", p1.x, p1.y);

    let p2 = p1.add(&p1);
    // let p2 = p1.add(&p3); 여기는 에러가 발생하는데 p1의 값은 i32이고 p3의 값은 f64이기 때문이다. T로 타입을 통일했기 때문이다.
    println!("p2=({},{})", p2.x, p2.y);
}

struct Point<T> {
    x: T,
    y: T,
}

// 모든 T에 대해, Point<T> 타입에 메서드를 정의를 의미
impl<T> Point<T> {
    fn add(&self, rhs: &Point<T>) -> Point<T>
    where
        T: std::ops::Add<Output = T> + Copy,
    {
        let x_val = self.x + rhs.x;
        let y_val = self.y + rhs.y;
        return Point { x: x_val, y: y_val };
    }
}

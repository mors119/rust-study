// Rust 기본 라이브러리에 정의된 트래잇을 impl하는 경우
#[derive(Clone, Debug)] // 트레잇 자동구현
struct Point {
    x: i32,
    y: i32,
}

//  Clone을 직접 구현
// impl Clone for Point {
//     fn clone(&self) -> Self {
//         Self {
//             x: self.x.clone(),
//             y: self.y.clone(),
//         }
//     }
// }

// Point 타입에 + 연산을 사용할 수 있도록 Add trait 구현
impl std::ops::Add for &Point {
    // 연산 결과 타입 정의 (Point + Point → Point)
    type Output = Point; // 결과는 새 Point

    // 실제 + 연산이 수행되는 함수
    fn add(self, rhs: Self) -> Self::Output {
        // 각 좌표를 더해서 새로운 Point 생성
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

// 새로운 트레잇을 만들어서 정의하기
trait MyOp {
    type Output;
    fn add(&self, rhs: &Self) -> Self::Output;
    fn sub(&self, rhs: &Self) -> Self::Output;
}

impl MyOp for Point {
    type Output = Self;

    fn add(&self, rhs: &Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }

    fn sub(&self, rhs: &Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

pub fn run() {
    let p1 = Point { x: 2, y: 3 };
    let p2 = Point { x: 4, y: 5 };

    let p3 = &p1 + &p2; // 참조로 넘기기
    let _p_clone = p1.clone() - p2.clone(); // clone
    let p4 = p1 - p2; // 소유권 이전
    println!(
        "addition=({},{}) subtraction=({},{})",
        p3.x, p3.y, p4.x, p4.y
    );

    // 새로운 트레잇을 만들어서 사용
    let p5 = p3.add(&p4);
    let p6 = p3.sub(&p4);
    println!("addition=({},{})", p5.x, p5.y);
    println!("subtraction=({},{})", p6.x, p6.y);

    print_distance(&p5);
}

// len은 없는 함수도 있기 때문에 직접 구현해야한다.
trait PointOp {
    fn len(&self) -> f64;
}

impl PointOp for Point {
    fn len(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}

// trait bound : 제네릭 타입중에서 지정된 Trait를 구현한 객체만으로 한정하는 것
fn print_distance<T: PointOp>(p: &T) {
    println!("Distance={}", p.len());
}

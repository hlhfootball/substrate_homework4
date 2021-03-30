<1.>
enum traffic_light {
    Red,
    Green,
    Yellow,
}

impl traffic_light {
    fn time(&self) -> u8 {
        match &self {
            traffic_light::Red => {
                50
            },
            traffic_light::Green => {
                60
            },
            traffic_light::Yellow => {
                70
            },
            _ => {
                10
            }
        }
    }
}
fn main() {
    let light = traffic_light :: Red;
    println!("light is {}",light.time());

    let light = traffic_light :: Yellow;
    println!("light is {}",light.time());

    let light = traffic_light :: Green;
    println!("light is {}",light.time());

}

<2.>
fn main() {
    let v= vec![0,1,2,5];

    println!("sum is {:?}",sum(&v));
}

fn sum(Vec:&[u32]) -> Option<u32>{
    let mut sum: Option<u32> = Some(0);
    
    for &i in Vec
    {
        sum = sum.unwrap().checked_add(i);
    }

    sum
} 

<3.>
fn main() {
    let square = Shape::Square(5_u32);
    let circle = Shape::Circle(6_f64,3.1415926_f64);
    let right_triangle = Shape::RightTriangle(2_u8,3_u8);

    println!("Square's area is {}", square.get_area());
    println!("Right Triangle's area is {}", right_triangle.get_area());
    println!("circle's area is {}", circle.get_area());
}

enum Shape<T> {
    Circle(T,T),
    Square(T),
    RightTriangle(T, T),
}

trait Area<T> {
    fn get_area(&self) -> T;
}

impl<T> Area<T> for Shape<T>
where
    T: std::ops::Mul<Output = T> + Clone + Copy,
{
    fn get_area(&self) -> T {
        match *self {
            Shape::Circle(a,pi) => a * a * pi,
            Shape::Square(a) => a * a,
            Shape::RightTriangle(a, b) => a * b,
        }
    }
}

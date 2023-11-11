#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl Point<f32, f32> {
    fn distance(&self, other: &Point<f32, f32>) -> f32 {
        f32::sqrt(f32::powi(self.x - other.x, 2) + f32::powi(self.y - other.y, 2))
    }
}

fn main() {
    let list_1 = vec![23, 13, 69, 42];
    let list_2 = vec!['f', 'a', 'y', 'r'];

    let largest_in_list_1 = largest(&list_1);
    let largest_in_list_2 = largest(&list_2);

    println!("The largest number is {largest_in_list_1}");
    println!("The largest char is {largest_in_list_2}");

    println!("-----");

    let point_int = Point { x: 23, y: 69 };
    let point_float = Point { x: 13.42, y: 23.69 };
    let point_str = Point { x: "lat", y: "lng" };

    let point_mixed = Point { x: 23, y: 69.13 };

    println!("point_int is: {:?}", point_int);
    println!("point_float is: {:?}", point_float);
    println!("point_str is: {:?}", point_str);
    println!("point_mixed is: {:?}", point_mixed);

    let point_1 = Point { x: 23.2, y: 13.3 };
    let point_2 = Point { x: 42.9, y: 69.6 };
    let distance = point_1.distance(&point_2);

    println!("distance: {distance}");
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

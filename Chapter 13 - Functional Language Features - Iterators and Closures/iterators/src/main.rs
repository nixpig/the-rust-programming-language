#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|shoe| shoe.size == size).collect()
}

fn main() {
    let v1 = vec![23, 42, 69];

    let v1_iter_loop = v1.iter();

    for val in v1_iter_loop {
        println!("val: {}", val);
    }

    let mut v1_iter_next = v1.iter();

    let a = v1_iter_next.next();
    let b = v1_iter_next.next();
    let c = v1_iter_next.next();
    let d = v1_iter_next.next();

    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let v1_iter_sum = v1.iter();

    let v1_total: u32 = v1_iter_sum.sum();

    println!("sum: {}", v1_total);

    let v1_iter_map = v1.iter();

    let min = 40;
    let add = 1;

    let v1_iter_mapped: Vec<_> = v1_iter_map.map(|x| x + add).filter(|y| y > &min).collect();
    println!("{:?}", v1_iter_mapped);
    for val in v1_iter_mapped.iter() {
        println!("mapped: {}", val);
    }

    println!("----------");

    let shoes = vec![
        Shoe {
            size: 9,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("loafer"),
        },
        Shoe {
            size: 10,
            style: String::from("sandal"),
        },
    ];

    let shoes_in_my_size = shoes_in_size(shoes, 9);
    println!("Shoes in my size: {:?}", shoes_in_my_size);
}

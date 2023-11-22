pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: vec![],
            average: 0 as f64,
        }
    }
    pub fn push(&mut self, x: i32) {
        self.list.push(x);
        self.update_average();
    }

    pub fn pop(&mut self) -> i32 {
        let val = self.list.pop();
        self.update_average();

        val.unwrap()
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

fn main() {
    let mut a = AveragedCollection::new();

    a.push(1);
    a.push(3);
    a.push(5);

    println!("average: {}", a.average());

    let popped = a.pop();
    println!("popped: {}", popped);

    println!("average: {}", a.average());
}

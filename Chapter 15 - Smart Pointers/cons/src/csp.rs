#[derive(Debug)]
pub struct CustomSmartPointer {
    data: String,
}

impl CustomSmartPointer {
    pub fn new(s: String) -> Self {
        Self { data: s }
    }

    pub fn data(&self) -> String {
        self.data.clone()
    }
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping: {:?}", self.data);
    }
}

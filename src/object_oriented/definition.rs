//! src/object_oriented/definition.rs
pub fn index(show: bool) {
    if show {
        definition();
    }
}

fn definition() {
    let mut average_collection = AveragedCollection {
        list: vec![1, 2, 3, 4, 5, 6, 7],
        average: 0.0,
    };

    average_collection.add(8);
    average_collection.add(9);
    average_collection.add(10);

    average_collection.remove();
    println!("Average: {}", average_collection.get_average());
}

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();

        match result {
            Some(value) => {
                self.update_average();

                Some(value)
            }
            None => None,
        }
    }

    pub fn get_average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

//! src/data_collections/definition.rs
struct Temperature {
    degrees_f: f64,
}

/*fn show_temp(temp: Temperature) {
    println!("{:?} degrees F", temp.degrees_f);
}*/

// this works for structs and enumerations
impl Temperature {
    /*fn show_temp(temp: Temperature) {
        println!("{:?} degrees F", temp.degrees_f);
    }*/

    // we don't need the call the function parameter as Temperature
    // we can use &self, as borrowing or in this case referencing
    fn show_temp(&self) {
        // here &self means we have a Temperature created somewhere
        println!("{:?} degrees F", self.degrees_f);
    }

    fn freezing() -> Self {
        // Self here means we are creating a new Temperature or we are referring as Temperature
        Self { degrees_f: 32.0 } // Use Self because if in the future you change the name of Temperature struct name
                                 // you don't have to change the implementation here
    }

    fn boiling() -> Self {
        Self { degrees_f: 212.0 }
    }
}

pub fn index(show: bool) {
    if show {
        println!("\n--- Definition");

        let hot = Temperature { degrees_f: 99.9 };
        // the show_temp function was moved as a function of Temperature struct
        //show_temp(hot);

        //Temperature::show_temp(&hot); this works but is better call the function because
        // hot is of type Temperature struct
        hot.show_temp();

        let cold = Temperature::freezing();
        cold.show_temp();

        let boiling = Temperature::boiling();
        boiling.show_temp();
    }
}

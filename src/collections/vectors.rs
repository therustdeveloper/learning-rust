//! src/collections/vectors.rs
pub fn index(show: bool) {
    if show {
        println!("--- Vectors ---");
        let v = vec![1, 2, 3, 4, 5, 6, 7];
        println!("The value of v is: {:?}", v);

        let v = vec![1, 2, 3, 4, 5, 6, 7]; //Vec::new();
                                           /*v.push(1);
                                           v.push(2);
                                           v.push(3);
                                           v.push(5);
                                           v.push(7);*/
        println!("The value of v is: {:?}", v);

        for i in &v {
            println!("{i}");
        }
    }
}

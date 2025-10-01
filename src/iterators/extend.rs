//! src/iterators/extend.rs
pub fn index(show: bool) {
    if show {
        println!("--- Iterating over Option");

        let grade: Option<&str> = Some("A+");
        let mut grades = vec!["B-", "C+", "D"];

        grades.extend(grade);

        println!("grades values: {:?}", grades);
    }
}

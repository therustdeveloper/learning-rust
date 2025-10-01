//! src/iterators/filter_map.rs
struct Company {
    name: String,
    ceo: Option<String>,
}

impl Company {
    fn new(name: &str, ceo: &str) -> Self {
        let ceo = match ceo {
            "" => None,
            ceo => Some(ceo.to_string()),
        };
        Self {
            name: name.to_string(),
            ceo,
        }
    }

    fn get_ceo(&self) -> Option<String> {
        self.ceo.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}

pub fn index(show: bool) {
    if show {
        println!("---Filter Map");

        let company_vec = [
            Company::new("Umbrella Corporation", "Unknown"),
            Company::new("Ovintiv", "Brendan McCracken"),
            Company::new("The Red-Headed League", ""),
            Company::new("Stark Enterprises", ""),
        ];

        let all_the_ceos = company_vec
            .iter()
            .filter_map(|company| company.get_ceo().map(|ceo| (company.get_name(), ceo)))
            .collect::<Vec<(String, String)>>();
        println!("{:?}", all_the_ceos);
    }
}

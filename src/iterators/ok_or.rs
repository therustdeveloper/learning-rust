//! src/iterators/ok_or.rs
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
        println!("---Convert OK to Result\n");

        let company_vec = vec![
            Company::new("Umbrella Corporation", "Unknown"),
            Company::new("Ovintiv", "Brendan McCracken"),
            Company::new("The Red-Headed League", ""),
            Company::new("Stark Enterprises", ""),
        ];

        let results: Vec<Result<(String, String), &str>> = company_vec
            .iter()
            .map(|company| {
                let name = company.get_name();
                let ceo = company.get_ceo().unwrap_or("CEO not available".to_string());
                Ok((name, ceo))
            })
            .collect();

        for item in results {
            match item {
                Ok((name, ceo)) => println!("Company: {}, CEO: {}", name, ceo),
                Err(e) => println!("Error: {}", e),
            }
        }
    }
}

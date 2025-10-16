use std::io::stdout;

struct Employee {
    pub name: String,
    pub id: u32,
}

::cttm::import!();

fn main() {
    println!("--- star.ct ---");

    cttm::tpl::star(&mut stdout(), 10).unwrap();

    println!("--- html.ct ---");

    cttm::tpl::html(&mut stdout(), &[
        Employee{
            name: "Alice".into(),
            id: 0,
        },
        Employee{
            name: "Bob".into(),
            id: 1
        }
    ]).unwrap();
}

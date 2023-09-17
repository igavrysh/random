// Topic: Lifetimes & Structures
//
// Requirements:
// * Display just the names and titles of persons from the mock-data.csv file
// * The names & titles must be stored in a struct separately from the mock
//   data for potential later usage
// * None of the mock data may be duplicated in memory
//
// Notes:
// * The mock data has already been loaded with the include_str! macro, so all functionality
//   must be implemented using references/borrows

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    title: &'a str,
}

fn load_persons_from_mock_data<'a>(lines: &'a str) -> Vec<Person<'a>> {
    lines.split('\n')
        .skip(1)
        .into_iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() < 5 {
                return None;
            }
            let name = parts[1];
            let title = parts[4];
            Some(Person { name, title })
        })
        .filter_map(|p| p)
        .collect()
}

fn print_persons<'a>(p: Vec<Person<'a>>)  {
    for person in p.iter() {
        println!("{:?}", person)
    }
}

fn main() {
    let p = load_persons_from_mock_data(MOCK_DATA);
    print_persons(p);
}
use std::io;
use std::io::BufRead;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug)]
struct Range {
    low: u64,
    high: u64,
}

#[derive(Debug)]
struct Validator {
    field_ranges: HashMap<String, Vec<Range>>,
}

#[derive(Debug)]
struct Ticket {
    fields: Vec<u64>,
}

impl Validator {
    fn new() -> Validator {
        Validator{field_ranges: HashMap::new()}
    }

    fn add_range(&mut self, field_name: String, low: u64, high: u64) {
        match self.field_ranges.get_mut(&field_name) {
            Some(ranges) => {
                ranges.push(Range{low, high});
            },
            None => {
                self.field_ranges.insert(field_name, vec![Range{low, high}]);
            },
        }
    }

    fn trivial_scanning_error(&self, ticket: Ticket) -> u64 {
        //For each ticket field
        ticket.fields.iter().fold(0, |sum, &field_value| {
            //Check to see if the field fails all available ranges
            let fails_all_field_ranges = self.field_ranges.values().fold(true, |is_failing, ranges| {
                is_failing & ranges.iter().fold(true, |failing_field, range| {
                    failing_field & (range.low > field_value || range.high < field_value)
                })
            });

            sum + if fails_all_field_ranges { field_value } else { 0 }
        })
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin()
        .lock()
        .lines()
        .for_each(|line| {
            buffer.push_str(&line.unwrap());
            buffer.push_str("\n");
        });

    let sections: Vec<&str> = buffer.split("\n\n").collect();
    let mut validator = Validator::new();

    sections[0].split("\n").for_each(|line| {
        let field_sections: Vec<&str> = line.split(": ").collect();
        let field_name = field_sections[0];

        field_sections[1].split(" or ").for_each(|range| {
            let range_parts: Vec<&str> = range.split("-").collect();
            let low = u64::from_str(&range_parts[0].trim()).unwrap();
            let high = u64::from_str(&range_parts[1].trim()).unwrap();
            validator.add_range(field_name.to_string(), low, high);
        });
    });

    //Ignore secitons[1] for now

    let mut nearby_tickets: Vec<&str> = sections[2].split("\n").collect();
    nearby_tickets.remove(0); //Remove "nearby tickets:" line
    nearby_tickets.remove(nearby_tickets.len()-1); //Remove "" line

    let count = nearby_tickets.iter().fold(0, |acc, line| {
        let ticket_fields: Vec<u64> = line.split(",")
            .map(|x| u64::from_str(&x.trim()).unwrap())
            .collect();

        acc + validator.trivial_scanning_error(Ticket{fields: ticket_fields})
    });

    println!("{}", count);

    /*let mut validator = Validator::new();
    validator.add_range("class".to_string(), 1, 3);
    validator.add_range("class".to_string(), 5, 7);
    validator.add_range("row".to_string(), 6, 11);
    validator.add_range("row".to_string(), 33, 44);
    validator.add_range("seat".to_string(), 13, 40);
    validator.add_range("seat".to_string(), 45, 50);

    println!("{:?}", validator);

    println!("{}", validator.trivial_scanning_error(Ticket{fields: vec![7, 3, 47]}));
    println!("{}", validator.trivial_scanning_error(Ticket{fields: vec![40, 4, 50]}));
    println!("{}", validator.trivial_scanning_error(Ticket{fields: vec![55, 2, 20]}));
    println!("{}", validator.trivial_scanning_error(Ticket{fields: vec![38, 6, 12]}));*/
}

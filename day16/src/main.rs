use std::io;
use std::io::BufRead;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug)]
struct Range {
    low: u64,
    high: u64,
}

impl Range {
    fn in_range(&self, num: u64) -> bool {
        self.low <= num && self.high >= num
    }
}

#[derive(Debug)]
struct Ticket {
    fields: Vec<u64>,
}

#[derive(Debug)]
struct Validator {
    field_ranges: HashMap<String, Vec<Range>>,
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

    fn get_ranges(&self, field_name: String) -> Option<&Vec<Range>> {
        self.field_ranges.get(&field_name)
    }

    fn trivial_scanning_error(&self, ticket: &Ticket) -> u64 {
        //For each ticket field
        ticket.fields.iter().fold(0, |sum, &field_value| {
            //Check to see if the field fails all available ranges
            let fails_all_field_ranges = self.field_ranges.values().fold(true, |is_failing, ranges| {
                is_failing & ranges.iter().fold(true, |failing_field, range| {
                    failing_field & !range.in_range(field_value)
                })
            });

            sum + if fails_all_field_ranges { field_value } else { 0 }
        })
    }

    fn is_valid_ticket(&self, ticket: &Ticket) -> bool {
        self.trivial_scanning_error(ticket) == 0
    }
}

#[derive(Debug)]
struct FieldLogic {
    validator: Validator,
    tickets: Vec<Ticket>,
}

impl FieldLogic {
    fn new(validator: Validator, tickets: Vec<Ticket>) -> FieldLogic {
        FieldLogic{
            validator: validator,
            tickets: tickets,
        }
    }

    fn solve(&self) -> HashMap<String, usize> {
        let cols_to_check: usize = self.tickets[0].fields.len();
        let field_names: Vec<String> = self.validator.field_ranges.keys().cloned().collect();
        let tickets = &self.tickets;

        let mut name_col_matrix = vec![vec![true; field_names.len()]; cols_to_check];
        //Basic plan:
        //Set name_col_matrix[i][j] to false when there's an inconsistency
        //between cols_to_check[i] and field_names[j]
        (0..cols_to_check).for_each(|col| {
            field_names.iter().enumerate().for_each(|(field_name_index, field_name)| {
                let ranges = self.validator.get_ranges(field_name.to_string()).unwrap();
                tickets.iter().for_each(|ticket| {
                    let in_range = ranges.iter().fold(false, |acc, range| {
                        acc | range.in_range(ticket.fields[col])
                    });
                    name_col_matrix[col][field_name_index] &= in_range;
                });
            });
        });

        let exit_condition = |ncm: &Vec<Vec<bool>>| {
            ncm.iter().fold(false, |acc1, row| {
                acc1 | row.iter().fold(false, |acc2, i| {
                    acc2 | i
                })
            })
        };

        let mut determined_fields = HashMap::new();
        while exit_condition(&name_col_matrix) {
            //Check by col
            for field_name_index in 0..name_col_matrix[0].len() {
                let mut true_count = 0;
                for col_on_ticket in 0..name_col_matrix.len() {
                    if name_col_matrix[col_on_ticket][field_name_index] {
                        true_count += 1;
                    }
                }

                if true_count != 1 {
                    continue;
                }

                let mut row_to_zero = 0;
                for col_on_ticket in 0..name_col_matrix.len() {
                    if name_col_matrix[col_on_ticket][field_name_index] {
                        row_to_zero = col_on_ticket;
                        break;
                    }
                }

                determined_fields.insert(field_names[field_name_index].to_string(), row_to_zero);

                //Zero the verified row
                for col in 0..name_col_matrix[0].len() {
                    name_col_matrix[row_to_zero][col] = false;
                }
            }

            //Check by row
            for col_on_ticket in 0..name_col_matrix.len() {
                let true_count = name_col_matrix[col_on_ticket].iter().fold(0, |acc, &i| {
                    if i { acc + 1 } else { acc }
                });

                if true_count != 1 {
                    break
                }

                let col_to_zero = name_col_matrix[col_on_ticket].iter().enumerate().fold(0, |acc, (col, &i)| {
                    if i { col } else { acc }
                });

                determined_fields.insert(field_names[col_to_zero].to_string(), col_on_ticket);

                //Zero the verified column
                for index in 0..name_col_matrix.len() {
                    name_col_matrix[index][col_to_zero] = false;
                }
            }
        }

        determined_fields
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

    let mut my_ticket: Vec<&str> = sections[1].split("\n").collect();
    my_ticket.remove(0); //Remove "your ticket:" line
    let my_ticket = Ticket{ fields: my_ticket[0]
        .split(",")
        .map(|x| u64::from_str(&x.trim()).unwrap())
        .collect()
    };

    let mut nearby_tickets: Vec<&str> = sections[2].split("\n").collect();
    nearby_tickets.remove(0); //Remove "nearby tickets:" line
    nearby_tickets.remove(nearby_tickets.len()-1); //Remove "" line

    let valid_tickets: Vec<Ticket> = nearby_tickets.iter()
        .map(|ticket|
            Ticket{ fields: ticket.split(",")
                .map(|x| u64::from_str(&x.trim()).unwrap())
                .collect()
            }
        )
        .filter(|ticket| validator.is_valid_ticket(ticket))
        .collect();


    let field_logic = FieldLogic::new(validator, valid_tickets);
    let solved_logic = field_logic.solve();

    let multiplied_solution = solved_logic.iter().fold(1, |acc, (field_name, &i)| {
        if field_name.starts_with("departure") { acc * my_ticket.fields[i] } else { acc }
    });
    println!("{}", multiplied_solution);
}

use std::io;
use std::io::BufRead;

#[derive(Clone,Debug,PartialEq)]
enum Seat {
    EMPTY,
    OCCUPIED,
    FLOOR,
}

use Seat::{EMPTY, OCCUPIED, FLOOR};

fn count_adjacent_occupied_seats(seats: &Vec<Vec<Seat>>, i: usize, j: usize) -> u8 {
    let mut count: u8 = 0;
    let ids: Vec<isize>;
    let jds: Vec<isize>;

    //Literal Edge Cases
    if i == 0 {
        ids = vec![0, 1];
    } else if i == seats.len() - 1 {
        ids = vec![-1, 0];
    } else {
        ids = vec![-1, 0, 1];
    }

    if j == 0 {
        jds = vec![0, 1];
    } else if j == seats[0].len() - 1 {
        jds = vec![-1, 0];
    } else {
        jds = vec![-1, 0, 1];
    }


    for id in &ids {
        for jd in &jds {
            let id = *id;
            let jd = *jd;
            if id == 0 && jd == 0 {
                continue;
            }

            count += match seats[((i as isize)+id) as usize][((j as isize)+jd) as usize] {
                OCCUPIED => 1,
                _ => 0
            };
        }
    }
    count
}

fn update_state(seats: &Vec<Vec<Seat>>) -> Vec<Vec<Seat>> {
    let mut updated_seats = seats.to_vec();
    for (i, row) in seats.iter().enumerate() {
        for (j, seat) in row.iter().enumerate() {
            let adj_count = count_adjacent_occupied_seats(seats, i, j);
            updated_seats[i][j] = match seat {
                EMPTY => if adj_count == 0 {OCCUPIED} else {EMPTY},
                OCCUPIED => if adj_count >= 4 {EMPTY} else {OCCUPIED},
                FLOOR => FLOOR,
            };
        }
    }
    updated_seats
}

fn count_seats(seats: &Vec<Vec<Seat>>) -> u64 {
    seats.iter().fold(0, |acc, row| {
        row.iter().fold(acc, |acc, seat| {
            if *seat == OCCUPIED {acc + 1} else {acc}
        })
    })
}

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut seats: Vec<Vec<Seat>> = vec![];
    stdin.lines().for_each(|line| {
        let line = line.unwrap();
        let mut row: Vec<Seat> = vec![];
        line.chars().for_each(|chr| {
            match chr {
                'L' => row.push(EMPTY),
                '.' => row.push(FLOOR),
                _ => (),
            };
        });
        seats.push(row);
    });

    loop {
        let prev_seats = seats;
        seats = update_state(&prev_seats);
        if seats.eq(&prev_seats) {
            break;
        }
    }
    println!("{:?}", count_seats(&seats));
}

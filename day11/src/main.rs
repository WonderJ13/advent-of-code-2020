use std::io;
use std::io::BufRead;

#[derive(Clone,Debug,PartialEq)]
enum Seat {
    EMPTY,
    OCCUPIED,
    FLOOR,
}

use Seat::{EMPTY, OCCUPIED, FLOOR};

fn in_bounds(i: isize, j: isize, i_max: usize, j_max: usize) -> bool {
    return i >= 0 && i < i_max as isize &&
            j >= 0 && j < j_max as isize
}

fn count_adjacent_occupied_seats(seats: &Vec<Vec<Seat>>, i: usize, j: usize) -> u8 {
    let mut count: u8 = 0;
    let i = i as isize;
    let j = j as isize;
    let ids: Vec<isize> = vec![-1, 0, 1];
    let jds: Vec<isize> = vec![-1, 0, 1];

    for id in &ids {
        for jd in &jds {
            let id = *id;
            let jd = *jd;
            if id == 0 && jd == 0 {
                continue;
            }

            let mut ii = i + id;
            let mut ji = j + jd;
            while in_bounds(ii, ji, seats.len(), seats[0].len()) &&
                seats[ii as usize][ji as usize] == FLOOR {
                ii += id;
                ji += jd;
            }
            if !in_bounds(ii, ji, seats.len(), seats[0].len()) {
                continue;
            }

            count += match seats[ii as usize][ji as usize] {
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
                OCCUPIED => if adj_count >= 5 {EMPTY} else {OCCUPIED},
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

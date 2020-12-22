use std::io;
use std::io::BufRead;

#[derive(Clone,Debug,PartialEq)]
enum Cube {
    INACTIVE,
    ACTIVE,
}

use Cube::{INACTIVE, ACTIVE};

type CubeRegion = Vec<Vec<Vec<Vec<Cube>>>>;

fn expand_region_x_dir(cubes: &mut CubeRegion) {
    let mut expand_left = false;
    let mut expand_right = false;

    let x_dir_count = cubes.len();
    let y_dir_count = cubes[0].len();
    let z_dir_count = cubes[0][0].len();
    let w_dir_count = cubes[0][0][0].len();

    for j in 0..y_dir_count {
        for k in 0..z_dir_count {
            for l in 0..w_dir_count {
                expand_left |= cubes[0][j][k][l] == ACTIVE;
                expand_right |= cubes[x_dir_count-1][j][k][l] == ACTIVE;
            }
        }
    }

    if expand_left {
        cubes.insert(0, vec![vec![vec![INACTIVE; w_dir_count]; z_dir_count]; y_dir_count]);
    }
    
    if expand_right {
        cubes.push(vec![vec![vec![INACTIVE; w_dir_count]; z_dir_count]; y_dir_count]);
    }
}

fn expand_region_y_dir(cubes: &mut CubeRegion) {
    let mut expand_top = false;
    let mut expand_bottom = false;


    let x_dir_count = cubes.len();
    let y_dir_count = cubes[0].len();
    let z_dir_count = cubes[0][0].len();
    let w_dir_count = cubes[0][0][0].len();

    for i in 0..x_dir_count {
        for k in 0..z_dir_count {
            for l in 0..w_dir_count {
                expand_top |= cubes[i][0][k][l] == ACTIVE;
                expand_bottom |= cubes[i][y_dir_count-1][k][l] == ACTIVE;
            }
        }
    }

    if expand_top {
        for i in 0..x_dir_count {
            cubes[i].insert(0, vec![vec![INACTIVE; w_dir_count]; z_dir_count]);
        }
    }

    if expand_bottom {
        for i in 0..x_dir_count {
            cubes[i].push(vec![vec![INACTIVE; w_dir_count]; z_dir_count]);
        }
    }
}

fn expand_region_z_dir(cubes: &mut CubeRegion) {
    let mut expand_front = false;
    let mut expand_back = false;


    let x_dir_count = cubes.len();
    let y_dir_count = cubes[0].len();
    let z_dir_count = cubes[0][0].len();
    let w_dir_count = cubes[0][0][0].len();

    for i in 0..x_dir_count {
        for j in 0..y_dir_count {
            for l in 0..w_dir_count {
                expand_front |= cubes[i][j][0][l] == ACTIVE;
                expand_back |= cubes[i][j][z_dir_count-1][l] == ACTIVE;
            }
        }
    }

    if expand_front {
        for i in 0..x_dir_count {
            for j in 0..y_dir_count {
                cubes[i][j].insert(0, vec![INACTIVE; w_dir_count]);
            }
        }
    }

    if expand_back {
        for i in 0..x_dir_count {
            for j in 0..y_dir_count {
                cubes[i][j].push(vec![INACTIVE; w_dir_count]);
            }
        }
    }
}

fn expand_region_w_dir(cubes: &mut CubeRegion) {
    let mut expand_flarg = false;
    let mut expand_blarg = false;


    let x_dir_count = cubes.len();
    let y_dir_count = cubes[0].len();
    let z_dir_count = cubes[0][0].len();
    let w_dir_count = cubes[0][0][0].len();

    for i in 0..x_dir_count {
        for j in 0..y_dir_count {
            for k in 0..z_dir_count {
                expand_flarg |= cubes[i][j][k][0] == ACTIVE;
                expand_blarg |= cubes[i][j][k][w_dir_count-1] == ACTIVE;
            }
        }
    }

    if expand_flarg {
        for i in 0..x_dir_count {
            for j in 0..y_dir_count {
                for k in 0..z_dir_count {
                    cubes[i][j][k].insert(0, INACTIVE);
                }
            }
        }
    }

    if expand_blarg {
        for i in 0..x_dir_count {
            for j in 0..y_dir_count {
                for k in 0..z_dir_count {
                    cubes[i][j][k].push(INACTIVE);
                }
            }
        }
    }
}

fn in_bounds(cubes: &CubeRegion, i: isize, j: isize, k: isize, l: isize) -> bool {
    i >= 0 && i < cubes.len() as isize &&
        j >= 0 && j < cubes[0].len() as isize &&
        k >= 0 && k < cubes[0][0].len() as isize &&
        l >= 0 && l < cubes[0][0][0].len() as isize
}

fn count_adjacent_cubes(cubes: &CubeRegion, i: usize, j: usize, k: usize, l: usize) -> u8 {
    let mut count: u8 = 0;
    let i = i as isize;
    let j = j as isize;
    let k = k as isize;
    let l = l as isize;

    let ids: Vec<isize> = vec![-1, 0, 1];
    let jds: Vec<isize> = vec![-1, 0, 1];
    let kds: Vec<isize> = vec![-1, 0, 1];
    let lds: Vec<isize> = vec![-1, 0, 1];

    for id in &ids {
        let id = *id;
        for jd in &jds {
            let jd = *jd;
            for kd in &kds {
                let kd = *kd;
                for ld in &lds {
                    let ld = *ld;
                    if id == 0 && jd == 0 && kd == 0 && ld == 0 {
                        continue;
                    }

                    let ii = i + id;
                    let ji = j + jd;
                    let ki = k + kd;
                    let li = l + ld;

                    if !in_bounds(cubes, ii, ji, ki, li) {
                        continue;
                    }

                    count += match cubes[ii as usize][ji as usize][ki as usize][li as usize] {
                        ACTIVE => 1,
                        _ => 0,
                    };
                }
            }
        }
    }
    count
}

fn perform_cycle(cubes: &CubeRegion) -> CubeRegion {
    let mut new_cubes = cubes.clone();
    expand_region_x_dir(&mut new_cubes);
    expand_region_y_dir(&mut new_cubes);
    expand_region_z_dir(&mut new_cubes);
    expand_region_w_dir(&mut new_cubes);

    let ref_cubes = new_cubes.clone();
    for i in 0..new_cubes.len() {
        for j in 0..new_cubes[0].len() {
            for k in 0..new_cubes[0][0].len() {
                for l in 0..new_cubes[0][0][0].len() {
                    let count = count_adjacent_cubes(&ref_cubes, i, j, k, l);
                    new_cubes[i][j][k][l] = match ref_cubes[i][j][k][l] {
                        ACTIVE => if count == 2 || count == 3 { ACTIVE } else { INACTIVE },
                        INACTIVE => if count == 3 { ACTIVE } else { INACTIVE },
                    };
                }
            }
        }
    }
    new_cubes
}

fn count_active_cubes(cubes: &CubeRegion) -> u64 {
    let mut count = 0;
    for i in 0..cubes.len() {
        for j in 0..cubes[0].len() {
            for k in 0..cubes[0][0].len() {
                for l in 0..cubes[0][0][0].len() {
                    count += match cubes[i][j][k][l] {
                        ACTIVE => 1,
                        INACTIVE => 0,
                    };
                }
            }
        }
    }
    count
}

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut cubes: CubeRegion = Vec::new();
    stdin.lines().for_each(|line| {
        let line = line.unwrap();
        let mut row: Vec<Vec<Vec<Cube>>> = vec![vec![vec![]]];
        line.chars().for_each(|chr| {
            let state = match chr {
                '#' => ACTIVE,
                _ => INACTIVE,
            };
            row[0][0].push(state);
        });
        cubes.push(row);
    });

    cubes = perform_cycle(&cubes);
    cubes = perform_cycle(&cubes);
    cubes = perform_cycle(&cubes);
    cubes = perform_cycle(&cubes);
    cubes = perform_cycle(&cubes);
    cubes = perform_cycle(&cubes);
    println!("{}", count_active_cubes(&cubes));
}

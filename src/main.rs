use rand::{distributions::Standard, Rng};

fn main() {
    println!("working on mountain and spiral traversal problems..");
    let traversal_matrix = prepare_matrix_for_traversal(12, 12);
    let result = traverse_spirally(traversal_matrix, 12, 12);
    println!("result: {:?}", result);
}

fn find_largest_mountain(
    n: usize, 
    range: u32, 
    v: Option<Vec<u32>>
) -> Vec::<u32> {
    let mut vec: Vec<u32>;
    if v.is_some() {
        vec = v.unwrap();
    } else {
        vec = prepare_random_vec_for_mountain(n, range)
    }
    let mut result = Vec::<u32>::new();
    let len: usize = vec.len();
    let mut i = 0;
    let mut tmp = Vec::<u32>::new();
    println!("vec: {:?}", vec);
    let mut max = 0;
    loop {
        println!("tmp: {:?}", tmp);
        if(i+1 == len) {
            break;
        }
        loop {
            // find all lesser values until logic shifts or end of array
            if i+1 == len {
                break;
            }
            println!("[L] cur: {} next: {}", vec[i], vec[i+1]);

            if vec[i] < vec[i+1] {
                tmp.push(vec[i]);
            } else {
                break;
            }
            i += 1;
        }

        if tmp.len() == 0 {
            i += 1;
            continue;
        }

        loop {
            // find all greater values until logic shifts or end of array
            if i+1 == len {
                break;
            }

            if vec[i] > vec[i+1] {
                tmp.push(vec[i]);
            } else {
                break;
            }
            
            i += 1;
        }
        tmp.push(vec[i]);
        
        let t_len = tmp.len() - 1;
        if tmp.len() < 3 || tmp[t_len] > tmp[t_len-1] {
            tmp.clear();
        }
        if tmp.len() != 0 {
            // if current mountain has len more than what is 
            // already there then switch to that!
            if t_len+1 > max {
                max = t_len + 1;
                result = tmp.clone();
            }
            // get tmp vec ready for the next round
            tmp.clear();
        }
    }
    println!("result: {:?}", result);
    result
}

enum SENSE {
    RIGHT,
    DOWN,
    LEFT,
    UP,
}

fn traverse_spirally(
    d: Vec<Vec<u32>>, 
    col_max: usize, 
    row_max: usize
) -> Vec<u32> {
    // counted (clockwise) from left -> bottom -> right -> top
    let mut cur_sense: SENSE = SENSE::RIGHT;

    let mut min_x = 1;
    let mut min_y = 0;
    let mut max_x = row_max - 1;
    let mut max_y = col_max - 1;
    let mut x = 0;
    let mut y = 0;
    let mut ctr = col_max * row_max;
    let mut result = Vec::<u32>::new();
    loop {
        loop {
            ctr -= 1;
            // println!("x: {}, y: {} elem: {}", x, y, d[x][y]);
            // println!("min_x: {} min_y: {} max_x: {} max_y: {} x: {}, y: {} c: {}", min_x, min_y, max_x, max_y, x, y, ctr);
            result.push(d[x][y]);

            match cur_sense {
                SENSE::RIGHT => {
                    // x, y++
                    if y == max_y {
                        // change sense
                        cur_sense = SENSE::DOWN;
                        // inc x for next sense
                        x += 1;
                        max_y -= 1;
                        break; // out of inner loop
                    }
                    y += 1;
                }
                SENSE::DOWN => {
                    // x++, y
                    if x == max_x {
                        // change sense
                        cur_sense = SENSE::LEFT;
                        // dec y for next sense
                        y -= 1;
                        max_x -= 1;
                        break; // out of inner loop
                    }
                    x += 1;
                }
                SENSE::LEFT => {
                    // x, y--
                    if y == min_y {
                        // change sense to up
                        cur_sense = SENSE::UP;
                        // inc y for next sense (ie up)
                        x -= 1;
                        min_y += 1;
                        break; // out of inner loop
                    }
                    y -= 1;
                }
                SENSE::UP => {
                    // x--, y
                    if x == min_x {
                        // change sense to right
                        cur_sense = SENSE::RIGHT;
                        // inc y for next sense (ie right)
                        y += 1;
                        min_x += 1;
                        break;
                    }
                    x -= 1;
                }
            }
        }
        // println!("");
        if ctr == 0 {
            // traversal completed
            break;
        }
    }
    // println!("result: {:?}", result);
    result
}

fn prepare_random_vec_for_mountain(
    n: usize, 
    range: u32
) -> Vec<u32> {
    let mut rnd = rand::thread_rng();
    let mut tmp = Vec::<u32>::new();
    for _ in 0..n {
        tmp.push(rnd.gen_range(0..=range));
    }
    tmp
}

fn prepare_matrix_for_traversal(
    row_max: usize, 
    col_max: usize
) -> Vec<Vec<u32>> {
    let mut y: Vec<Vec<u32>> = Vec::new();
    let mut i = 0;
    let mut j = 0;
    let mut ctr = 1;
    while i < row_max {
        let mut tmp: Vec<u32> = Vec::new();
        while j < col_max {
            tmp.push(ctr);
            ctr += 1;
            j += 1;
        }
        j = 0;
        i += 1;
        y.push(tmp.clone());
    }
    y
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_working() {
        assert_eq!(1, 1);
    }

    #[test]
    fn check_prepare_matrix_working() {
        let matrix_4_x_4 = prepare_matrix_for_traversal(4, 4);
        assert_eq!(matrix_4_x_4.len(), 4);
        assert_eq!(matrix_4_x_4[0].len(), 4);
        assert_eq!(matrix_4_x_4[0][0], 1);
        assert_eq!(matrix_4_x_4[3][3], 16);
    }

    #[test]
    fn check_spiral_traversal_is_working_4_x_4() {
        const cols: usize = 4;
        const rows: usize = 4;
        let matrix_4_x_4 = prepare_matrix_for_traversal(rows, cols);
        let spiral_vec = traverse_spirally(matrix_4_x_4, rows, cols);
        assert_eq!(spiral_vec.len(), rows * cols);
        assert_eq!(spiral_vec[0], 1);
        assert_eq!(spiral_vec[rows * cols - 1], 10);
        assert_eq!(
            spiral_vec[..],
            [1, 2, 3, 4, 8, 12, 16, 15, 14, 13, 9, 5, 6, 7, 11, 10]
        );
    }

    #[test]
    fn check_spiral_traversal_is_working_12_x_12() {
        const cols: usize = 12;
        const rows: usize = 12;
        let matrix_12_x_12 = prepare_matrix_for_traversal(rows, cols);
        let spiral_vec = traverse_spirally(matrix_12_x_12, rows, cols);
        assert_eq!(spiral_vec.len(), rows * cols);
        assert_eq!(spiral_vec[0], 1);
        assert_eq!(spiral_vec[rows * cols - 1], 78);
        assert_eq!(
            spiral_vec[..],
            [
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 24, 36, 48, 60, 72, 84, 96, 108, 120, 132,
                144, 143, 142, 141, 140, 139, 138, 137, 136, 135, 134, 133, 121, 109, 97, 85, 73,
                61, 49, 37, 25, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 35, 47, 59, 71, 83, 95,
                107, 119, 131, 130, 129, 128, 127, 126, 125, 124, 123, 122, 110, 98, 86, 74, 62,
                50, 38, 26, 27, 28, 29, 30, 31, 32, 33, 34, 46, 58, 70, 82, 94, 106, 118, 117, 116,
                115, 114, 113, 112, 111, 99, 87, 75, 63, 51, 39, 40, 41, 42, 43, 44, 45, 57, 69,
                81, 93, 105, 104, 103, 102, 101, 100, 88, 76, 64, 52, 53, 54, 55, 56, 68, 80, 92,
                91, 90, 89, 77, 65, 66, 67, 79, 78
            ]
        );
    }

    #[test]
    fn check_prepare_random_vec_working() {
        let rand_vec_10 = prepare_random_vec_for_mountain(10, 20);
        println!("vec: {:?}", rand_vec_10);
        assert_eq!(rand_vec_10.len(), 10);
    }

    #[test]
    fn check_find_largest_mountain_works() {
        let result = find_largest_mountain(0, 0, Some(vec![4, 1, 2, 7, 4, 3, 5, 6, 7, 8, 9, 8, 7, 6, 5, 1, 10]));
        assert_eq!(result.len(), 11);
        assert_eq!(result, [3, 5, 6, 7, 8, 9, 8, 7, 6, 5, 1])
    }
}

mod ui;

//use ui::gui::{ create_window };
//use ui::console::{ create_terminal };


fn main() {
    //create_window();

    let grid =[[3, 0, 6, 5, 0, 8, 4, 0, 0],
               [5, 2, 0, 0, 0, 0, 0, 0, 0],
               [0, 8, 7, 0, 0, 0, 0, 3, 1],
               [0, 0, 3, 0, 1, 0, 0, 8, 0],
               [9, 0, 0, 8, 6, 3, 0, 0, 5],
               [0, 5, 0, 0, 9, 0, 6, 0, 0],
               [1, 3, 0, 0, 0, 0, 2, 5, 0],
               [0, 0, 0, 0, 0, 0, 0, 7, 4],
               [0, 0, 5, 2, 0, 6, 3, 0, 0]];

    //let (ans, b) = solve_sudoku(grid);
    let (b, e) = find_empty(grid);
    println!("We have a solution? {:?}", );
    
}

type Sudoku = [[u8; 9]; 9];
type Elem = [usize; 2];

pub fn solve_sudoku(s: Sudoku) -> (Sudoku, bool) {
    let mut sudoku = s;
    let (empty_loc, is_empty) = find_empty(sudoku);

    // No more empty places. We done!
    if !is_empty {
        return (sudoku, true);
    }

    for num in 1..10{
        if check_loc_safe(sudoku, num, empty_loc){
            sudoku[empty_loc[1]][empty_loc[1]] = num;

            let (ans, b) = solve_sudoku(sudoku);
            if b {
                return (ans, true)
            }
            else {
                sudoku[empty_loc[0]][empty_loc[1]] = 0;
            }
        }
    }

    (sudoku, false)
}

pub fn used_in_row(sudoku: Sudoku, row: usize, e: u8) -> bool{
    if sudoku[row].contains(&e){
        return true;
    }

    false
}

pub fn used_in_col(sudoku: Sudoku, col: usize, e: u8) -> bool{
    for i in 0..9{
        if sudoku[i][col] == e{
            return true;
        }
    }

    false
}

pub fn used_in_box(sudoku: Sudoku, loc: Elem, e: u8) -> bool{
    let box_row = loc[0] - (loc[0] % 3);
    let box_col = loc[1] - (loc[1] % 3);

    for i in 0..3{
        for j in 0..3{
            if sudoku[i + box_row * 3][j + box_col] == e{
                return true;
            }
        }
    }

    false
}

pub fn find_empty(sudoku: Sudoku) -> (Elem, bool) {
    for i in 0..9{
        for j in 0..9{
            if sudoku[i][j] == 0 {
                return ([i, j], true);
            }
        }
    }

    ([0, 0], false)
}

pub fn check_loc_safe(sudoku: Sudoku, e: u8, loc: Elem) -> bool {
    !used_in_row(sudoku, loc[0], e)
        && !used_in_col(sudoku, loc[1], e)
        && !used_in_box(sudoku, loc, e)
}

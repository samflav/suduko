use crate::board;
use crate::board::Value::{NoInit, Set, Unset};

#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Set(char),
    Unset(Vec<char>),
    NoInit
}

impl Value {
    fn unwrap_unset(&self) -> Vec<char> {
        match self {
            Unset(val) => val.clone(),
            _=> panic!()
        }
    }
}

#[derive(Clone)]
pub struct Board {
    pub board: Vec<Value>
}

impl Board {
    const VALUES: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

    pub fn new(input: Vec<String>) -> Board {
        let mut vec_out = vec![];

        for line in input {
            for single in line.chars() {
                if single == '0' {
                    vec_out.push(NoInit)
                } else {
                    vec_out.push(Set(single))
                }
            }
        }

        Board::init(&mut Board {
            board: vec_out
        })
    }

    fn init(brd: &mut Board) -> Board {
        for (idx, val) in brd.board.clone().into_iter().enumerate() {
            if let NoInit = val {
                brd.update_possible(idx);
            }
        }

        brd.clone()
    }
}

impl Board {
    fn update_possible(&mut self, idx: usize) {
        let mut possible_vals = Board::VALUES.to_vec();
        let impossible_vals = Board::get_set_associated(self, idx);

        possible_vals.retain(|num| !impossible_vals.contains(num));

        self.board[idx] = Unset(possible_vals);
    }

    fn set_index(&mut self, idx: usize, val: char) -> bool {
        if self.get_set_associated(idx).contains(&val) {
            return false;
        }

        self.board[idx] = Set(val);

        for i in Board::get_associated_indexes(idx) {
            if let Unset(mut poss) = self.board[i].clone() {
                poss.retain(|num| num != &val);

                if poss.len() == 1 {
                    self.set_index(i, poss[0]);
                } else {
                    self.board[i] = Unset(poss);
                }
            }
        }

        true
    }

    pub fn trim(&mut self, iterations: usize) -> bool {

        for _iter in 0..iterations {
            self.set_singles();
            self.calc_only();
        }

        true
    }

    pub fn backtrace(&mut self, idx: usize) -> bool {
        if idx >= 81 {
            return true;
        }
        return match &self.board[idx] {
            Unset(_vec) => {
                for try_char in Board::VALUES.iter() {
                    if self.back_set(idx, try_char) {
                        if self.backtrace(idx + 1) {
                            return true;
                        }
                    }
                }
                self.board[idx] = Unset(['0'].to_vec());
                false
            },
            _ => {
                self.backtrace(idx + 1)
            }
        };
    }

    fn back_set(&mut self, idx: usize, val: &char) -> bool {
        if self.get_set_associated(idx).contains(&val) {
            return false;
        }

        self.board[idx] = Set(*val);

        true
    }

    pub fn set_singles(&mut self) -> bool {

        for (idx, val) in self.board.clone().iter().enumerate() {
            if let Unset(poss) = val.clone() {
                if poss.len() == 1 {
                    if !self.set_index(idx, poss[0]) {
                        return false;
                    }
                }
            }
        }

        true
    }

    pub fn calc_only(&mut self) -> bool {

        for index in 0..9 {
            let row = index * 9;
            let col = index;
            let square = (((index / 3) * 9) * 3) + ((index % 3) * 3);

            if !self.set_only(&Board::get_row_indexes(row, true)) {
                return false;
            }

            if !self.set_only(&Board::get_col_indexes(col, true)) {
                return false;
            }

            if !self.set_only(&Board::get_square_indexes(square)) {
                return false;
            }
        }

        true
    }

    fn set_only(&mut self, group: &Vec<usize>) -> bool {
        let mut all_vals = vec![];

        for idx in group {
            if let Unset(vals) = &self.board[*idx] {
                vals.iter().for_each(|val| all_vals.push(*val))
            }
        }

        for idx in group {

                if let Unset(vals) = self.board[*idx].clone() {

                    vals.iter().for_each(|val| {

                        let match_vals: Vec<&char>= all_vals
                            .iter()
                            .filter(|&num| num == val)
                            .collect();

                        if match_vals.len() == 1 {
                            self.set_index(*idx, *val);
                        }
                    })
                }

        }

        true
    }

    pub fn calc_doubles(&mut self) -> bool {

        for index in 0..9 {
            let row = index * 9;
            let col = index;
            let square = (((index / 3) * 9) * 3) + ((index % 3) * 3);

            if !self.find_doubles(&Board::get_row_indexes(row, true)) {
                return false;
            }

            if !self.find_doubles(&Board::get_col_indexes(col, true)) {
                return false;
            }

            if !self.find_doubles(&Board::get_square_indexes(square)) {
                return false;
            }
        }

        true
    }

    fn find_doubles(&mut self, group: &Vec<usize>) -> bool {
            for (iter_index, board_index) in group.iter().enumerate() {

                let potential = self.board[*board_index].clone();

                if let Some(idx) = group[iter_index + 1..].iter()
                    .position(|inx| self.board[*inx] == potential) {

                    let pair = potential.unwrap_unset();

                    if pair.len() == 2 {

                        if !self.set_doubles(group, pair) {
                            return false
                        }

                    }
                }
            }

        true
    }

    fn set_doubles(&mut self, group: &Vec<usize>, pair: Vec<char>) -> bool {

        for index in group {

            if let Unset(mut nums) = self.board[*index].clone() {

                if nums != pair {

                    nums.retain(|num| num != &pair[0] && num != &pair[1]);

                    if nums.len() == 1 {

                         if !self.set_index(*index, nums[0]) {
                             return false;
                         }

                    } else {
                        self.board[*index] = Unset(nums);
                    }
                }
            }
        }


        true
    }
}

impl Board {
    fn read_indexes(&self, indexes: &Vec<usize>) -> Vec<Value> {
        let mut out = vec![];

        for idx in indexes.clone() {
            out.push(self.board[idx].clone())
        }

        out
    }

    fn get_set_associated(&self, idx: usize) -> Vec<char> {
        let mut out = vec![];

        for val in self.read_indexes(&Board::get_associated_indexes(idx)) {
            if let Set(num) = val {
                if !out.contains(&num) {
                    out.push(num);
                }
            }
        }

        out
    }

    fn get_col_indexes(idx: usize, include_index: bool) -> Vec<usize> {
        let mut out = vec![];

        let shift = idx % 9;

        for i in 0..9 {
            out.push((i * 9) + shift);
        }

        if !include_index {
            let row = idx / 9;
            out.remove(row);
        }

        out
    }

    fn get_row_indexes(idx: usize, include_index: bool) -> Vec<usize> {
        let start = (idx / 9) * 9;

        let mut out: Vec<usize> = (start..start + 9).collect();

        if !include_index {
            let actual_col = idx % 9;
            out.remove(actual_col);
        }

        out
    }

    fn get_square_indexes(idx: usize) -> Vec<usize> {
        let mut out = vec![];

        let start_row = ((idx / 9) / 3) * 3;
        let start_col = ((idx % 9) / 3) * 3;

        for row in start_row..start_row + 3 {
            for col in start_col..start_col + 3 {
                out.push((row * 9) + col);
            }
        }

        out
    }

    fn get_distinct_square_indexes(idx: usize) -> Vec<usize> {
        let mut out = vec![];

        let actual_row = idx / 9;
        let actual_col = idx % 9;

        let start_row = ((idx / 9) / 3) * 3;
        let start_col = ((idx % 9) / 3) * 3;

        for row in start_row..start_row + 3 {
            for col in start_col..start_col + 3 {
                if row == actual_row || col == actual_col {
                    continue;
                }
                out.push((row * 9) + col);
            }
        }

        out
    }

    fn get_associated_indexes(idx: usize) -> Vec<usize> {
        let mut vals = Board::get_row_indexes(idx, false);
        vals.extend(Board::get_col_indexes(idx, false));
        vals.extend(Board::get_distinct_square_indexes(idx));

        vals
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_board() -> Board {
        Board{
            board: vec![
                Set('1'), Set('2'), Set('3'), Set('4'), Set('5'), Set('6'), Set('7'), Set('8'), Set('9'),
                Set('2'), Set('3'), Set('4'), Set('5'), Set('6'), Set('7'), Set('8'), Set('9'), Set('1'),
                Set('3'), Set('4'), Set('5'), Set('6'), Set('7'), Set('8'), Set('9'), Set('1'), Set('2'),
                Set('4'), Set('5'), Set('6'), Set('7'), Set('8'), Set('9'), Set('1'), Set('2'), Set('3'),
                Set('5'), Set('6'), Set('7'), Set('8'), Set('9'), Set('1'), Set('2'), Set('3'), Set('4'),
                Set('6'), Set('7'), Set('8'), Set('9'), Set('1'), Set('2'), Set('3'), Set('4'), Set('5'),
                Set('7'), Set('8'), Set('9'), Set('1'), Set('2'), Set('3'), Set('4'), Set('5'), Set('6'),
                Set('8'), Set('9'), Set('1'), Set('2'), Set('3'), Set('4'), Set('5'), Set('6'), Set('7'),
                Set('a'), Set('b'), Set('c'), Set('d'), Set('e'), Set('f'), Set('g'), Set('h'), Set('i')
            ]
        }
    }

    fn different_board() -> Board {
        Board{
            board: vec![
                Set('1'), Set('2'), NoInit, Set('4'), Set('5'), NoInit, Set('7'), Set('8'), Set('9'),
                NoInit, Set('3'), Set('4'), Set('5'), Set('6'), Set('7'), Set('8'), Set('9'), Set('1'),
                Set('3'), Set('4'), Set('5'), Set('6'), Set('7'), Set('8'), Set('9'), Set('1'), Set('2'),
                Set('4'), Set('5'), Set('6'), Set('7'), Set('8'), Set('9'), Set('1'), Set('2'), Set('3'),
                Set('5'), Set('6'), Set('7'), Set('8'), Set('9'), Set('1'), Set('2'), Set('3'), Set('4'),
                NoInit, Set('7'), Set('8'), Set('9'), Set('1'), Set('2'), Set('3'), Set('4'), Set('5'),
                Set('7'), Set('8'), Set('9'), Set('1'), Set('2'), Set('3'), Set('4'), Set('5'), Set('6'),
                Set('8'), Set('9'), Set('1'), Set('2'), Set('3'), Set('4'), Set('5'), Set('6'), Set('7'),
                Set('a'), Set('b'), Set('c'), Set('d'), Set('e'), Set('f'), Set('g'), Set('h'), Set('i')
            ]
        }
    }

    // #[test]
    // fn row_test() {
    //     let brd = default_board();
    //
    //     let row = vec![
    //         Set('1'), Set('3'), Set('4'), Set('5'), Set('6'), Set('7'), Set('8'), Set('9')
    //     ];
    //
    //     let gen_row = brd.get_row(1);
    //
    //     for (idx, val) in row.iter().enumerate() {
    //         assert_eq!(*val, gen_row[idx]);
    //     }
    //
    //     let row = vec![
    //         Set('5'), Set('7'), Set('8'), Set('9'), Set('1'), Set('2'), Set('3'), Set('4')
    //     ];
    //
    //     let gen_row = brd.get_row((9 * 4) + 1);
    //
    //     for (idx, val) in row.iter().enumerate() {
    //         assert_eq!(*val, gen_row[idx]);
    //     }
    //
    //     let row = vec![
    //         Set('a'), Set('b'), Set('c'), Set('d'), Set('f'), Set('g'), Set('h'), Set('i')
    //     ];
    //
    //     let gen_row = brd.get_row((9 * 8) + 4);
    //
    //     for (idx, val) in row.iter().enumerate() {
    //         assert_eq!(*val, gen_row[idx]);
    //     }
    // }
    //
    // #[test]
    // fn col_test() {
    //     let brd = default_board();
    //
    //     let col = vec![
    //         Set('2'), Set('3'), Set('4'), Set('5'), Set('6'), Set('7'), Set('8'), Set('a')
    //     ];
    //
    //     let gen_col = brd.get_col(0);
    //
    //     for (idx, val) in col.iter().enumerate() {
    //         assert_eq!(*val, gen_col[idx]);
    //     }
    //
    //     let col = vec![
    //         Set('9'), Set('1'), Set('2'), Set('3'), Set('4'), Set('5'), Set('6'), Set('h')
    //     ];
    //
    //     let gen_col = brd.get_col(7);
    //
    //     for (idx, val) in col.iter().enumerate() {
    //         assert_eq!(*val, gen_col[idx]);
    //     }
    //
    //     let col = vec![
    //         Set('4'), Set('5'), Set('6'), Set('7'), Set('8'), Set('9'), Set('1'), Set('2')
    //     ];
    //
    //     let gen_col = brd.get_col((9 * 8) + 3);
    //
    //     for (idx, val) in col.iter().enumerate() {
    //         assert_eq!(*val, gen_col[idx]);
    //     }
    //
    //     let col = vec![
    //         Set('4'), Set('5'), Set('6'), Set('7'), Set('9'), Set('1'), Set('2'), Set('d')
    //     ];
    //
    //     let gen_col = brd.get_col((9 * 4) + 3);
    //
    //     for (idx, val) in col.iter().enumerate() {
    //         assert_eq!(*val, gen_col[idx]);
    //     }
    // }
    //
    // #[test]
    // fn square_test() {
    //     let brd = default_board();
    //
    //     let square = vec![
    //         Set('2'), Set('3'), Set('2'), Set('3'), Set('4'), Set('3'), Set('4'), Set('5')
    //     ];
    //
    //     let gen_col = brd.get_square(0);
    //
    //     println!("{:?}", gen_col);
    //
    //     for (idx, val) in square.iter().enumerate() {
    //         assert_eq!(*val, gen_col[idx]);
    //     }
    //
    //     let square = vec![
    //         Set('1'), Set('2'), Set('3'), Set('2'), Set('3'), Set('3'), Set('4'), Set('5')
    //     ];
    //
    //     let gen_col = brd.get_square(11);
    //
    //     println!("{:?}", gen_col);
    //
    //     for (idx, val) in square.iter().enumerate() {
    //         assert_eq!(*val, gen_col[idx]);
    //     }
    //
    //     let square = vec![
    //         Set('1'), Set('2'), Set('3'), Set('2'), Set('3'), Set('4'), Set('e'), Set('f')
    //     ];
    //
    //     let gen_col = brd.get_square((9 * 8) + 3);
    //
    //     println!("{:?}", gen_col);
    //
    //     for (idx, val) in square.iter().enumerate() {
    //         assert_eq!(*val, gen_col[idx]);
    //     }
    // }
    //
    // #[test]
    // fn distinct_square_test() {
    //     let brd = default_board();
    //
    //     let square = vec![
    //         Set('3'), Set('4'), Set('4'), Set('5')
    //     ];
    //
    //     let gen_col = brd.get_distinct_square(0);
    //
    //     println!("{:?}", gen_col);
    //
    //     for (idx, val) in square.iter().enumerate() {
    //         assert_eq!(*val, gen_col[idx]);
    //     }
    //
    //     let square = vec![
    //         Set('1'), Set('2'), Set('3'), Set('4')
    //     ];
    //
    //     let gen_col = brd.get_distinct_square(11);
    //
    //     println!("{:?}", gen_col);
    //
    //     for (idx, val) in square.iter().enumerate() {
    //         assert_eq!(*val, gen_col[idx]);
    //     }
    //
    //     let square = vec![
    //         Set('2'), Set('3'), Set('3'), Set('4')
    //     ];
    //
    //     let gen_col = brd.get_distinct_square((9 * 8) + 3);
    //
    //     println!("{:?}", gen_col);
    //
    //     for (idx, val) in square.iter().enumerate() {
    //         assert_eq!(*val, gen_col[idx]);
    //     }
    //
    //     let square = vec![
    //         Set('5'), Set('6'), Set('h'), Set('i')
    //     ];
    //
    //     let gen_col = brd.get_distinct_square((9 * 7) + 6);
    //
    //     println!("{:?}", gen_col);
    //
    //     for (idx, val) in square.iter().enumerate() {
    //         assert_eq!(*val, gen_col[idx]);
    //     }
    // }
    //
    // #[test]
    // fn associated_test() {
    //     let brd = default_board();
    //
    //     let vals = vec![
    //         Set('2'), Set('3'), Set('4'), Set('5'), Set('6'), Set('7'), Set('8'), Set('9'),
    //         Set('2'), Set('3'), Set('4'), Set('5'), Set('6'), Set('7'), Set('8'), Set('a'),
    //         Set('3'), Set('4'), Set('4'), Set('5')
    //     ];
    //
    //     let gen_vals = brd.get_associated(0);
    //
    //     for (idx, val) in vals.iter().enumerate() {
    //         assert_eq!(*val, gen_vals[idx]);
    //     }
    //
    //     let vals = vec![
    //         Set('6'), Set('7'), Set('8'), Set('1'), Set('2'), Set('3'), Set('4'), Set('5'),
    //         Set('4'), Set('5'), Set('6'), Set('7'), Set('8'), Set('1'), Set('2'), Set('d'),
    //         Set('8'), Set('9'), Set('9'), Set('1')
    //     ];
    //
    //     let gen_vals = brd.get_associated((9 * 5) + 3);
    //
    //     println!("{:?}", gen_vals);
    //
    //     for (idx, val) in vals.iter().enumerate() {
    //         assert_eq!(*val, gen_vals[idx]);
    //     }
    // }
    //
    // #[test]
    // fn set_associated_test() {
    //     let brd = default_board();
    //
    //     let vals = vec![
    //         '2', '3', '4', '5', '6', '7', '8', '9', 'a'
    //     ];
    //
    //     let gen_vals = brd.get_set_associated(0);
    //
    //     for (idx, val) in vals.iter().enumerate() {
    //         assert_eq!(*val, gen_vals[idx]);
    //     }
    //
    //     let brd = different_board();
    //
    //     let vals = vec![
    //         '2', '4', '5', '7', '8', '9', '3', 'a'
    //     ];
    //
    //     let gen_vals = brd.get_set_associated(0);
    //
    //     for (idx, val) in vals.iter().enumerate() {
    //         assert_eq!(*val, gen_vals[idx]);
    //     }
    // }
}
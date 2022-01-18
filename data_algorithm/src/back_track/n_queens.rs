use std::collections::HashMap;

struct Solution {}

///
/// n皇后问题 研究的是如何将 n个皇后放置在 n×n 的棋盘上，并且使皇后彼此之间不能相互攻击。
///
/// 给你一个整数 n ，返回所有不同的n皇后问题 的解决方案。
///
/// 每一种解法包含一个不同的 n 皇后问题 的棋子放置方案，该方案中 'Q' 和 '.' 分别代表了皇后和空位。
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/n-queens
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///
impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = Vec::new();
        let mut result1: HashMap<i32, i32> = HashMap::new();
        set_queen(n, &mut result1, &mut result, 0);
        return result;
    }
}

///进行queen的放入
fn set_queen(n: i32, queens: &mut HashMap<i32, i32>, queen_list: &mut Vec<Vec<String>>, row: i32) {
    if row == n {
        print_queens(queens, queen_list, n);
        return;
    }
    for col in 0..n {
        if is_ok(row, col, queens, n) {
            queens.insert(row, col);
            set_queen(n, queens, queen_list, row + 1)
        }
    }
}

/// 设置queen
fn print_queens(queens: &mut HashMap<i32, i32>, queen_list: &mut Vec<Vec<String>>, n: i32) {
    let mut result: Vec<String> = Vec::new();
    for i in 0..n {
        let mut s = String::new();
        for j in 0..n {
            if queens.get(&i).unwrap_or(&-1) == &j {
                s = s + "Q";
            } else {
                s = s + ".";
            }
        }
        result.push(s);
    }
    queen_list.push(result);
}

fn is_ok(row: i32, col: i32, queens: &mut HashMap<i32, i32>, n: i32) -> bool {
    let mut leftup = col - 1;
    let mut rightup = col + 1;

    let mut i = row - 1;
    while i >= 0 {
        //列上存在queen
        if queens.get(&i).unwrap_or(&-1) == &col {
            return false;
        }
        //左上对角线存在queen
        if leftup >= 0 {
            if queens.get(&i).unwrap_or(&-1) == &leftup {
                return false;
            }
        }
        //右下对角线存在queen
        if rightup < n {
            if queens.get(&i).unwrap_or(&-1) == &rightup {
                return false;
            }
        }
        leftup -= 1;
        rightup += 1;
        i = i - 1;
    }

    return true;
}

fn print_queen(re: &Vec<Vec<String>>) {
    for e in re {
        for i in e {
            print!("{} ", i)
        }
        println!()
    }
}

#[test]
fn parse_url_works() {
    let re: Vec<Vec<String>> = Solution::solve_n_queens(4);

    print_queen(&re);
}

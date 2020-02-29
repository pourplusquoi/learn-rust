use std::collections::HashSet;
use std::collections::HashMap;

fn main() {

    /*********************************************************************/
    /************************** User Input Area **************************/
    /*********************************************************************/

    let lhs = vec!["Pb(N2)3$", "Cr(MnO4)2$"];
    let rhs = vec!["Cr2O3$", "MnO2$", "Pb3O4$", "NO$"];

    /*********************************************************************/
    /*********************************************************************/
    /*********************************************************************/

    println!(">>> Input Parsed");
    println!("{}\n", compose_reaction(&lhs, &rhs));

    println!(">>> Solution Emitted");
    let equations = equalize(&lhs, &rhs);
    if equations.is_empty() {
        println!("Impossible.");
    } else {
        println!("Found {} linearly independent solution(s).", equations.len());
        for equation in equations {
            println!("{}", equation);
        }
    }
}

/******************************************************************************/
/********************************** Nullspace *********************************/
/******************************************************************************/

// Calculates the bases of nullspace for the given matrix.
fn nullspace(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    mat = reduce(mat);
    let mut bases: Vec<Vec<i32>> = Vec::new();
    let mut independent: HashSet<usize> = HashSet::new();
    for (idx, row) in mat.iter().enumerate() {
        if row[idx] == 0 {
            // Found a basis.
            independent.insert(idx);
        }
    }
    let rows = mat.len();
    let cols = mat[0].len();
    for idx in rows..cols {
        // More basis.
        independent.insert(idx);
    }
    for &idx in independent.iter() {
        bases.push(calculate_basis(&mat, &independent, idx));
    }
    bases
}

fn calculate_basis(mat: &Vec<Vec<i32>>,
                   independent: &HashSet<usize>,
                   selected_idx: usize) -> Vec<i32> {
    let cols = mat[0].len();
    let mut coefficients: Vec<i32> = Vec::new();
    for row in mat.iter() {
        for (col_idx, num) in row.iter().enumerate() {
            if !independent.contains(&col_idx) {
                coefficients.push(*num);
            }
        }
    }
    let mut basis = vec![0; cols];
    basis[selected_idx] = lcm(&coefficients);
    for row in mat.iter() {
        let mut lhs = 0;
        let mut rhs = 0;
        let mut var_idx = 0;
        for (col_idx, num) in row.iter().enumerate() {
            if col_idx == selected_idx {
                rhs = -num;
            } else if *num != 0 && !independent.contains(&col_idx) {
                lhs = *num;
                var_idx = col_idx;
            }
        }
        // Avoid deviding by zero.
        if lhs == 0 {
            panic!("The dimension of nullspace is 0.");
        }
        basis[var_idx] = rhs * basis[selected_idx] / lhs;
    }
    basis
}

// Reduces the matrix to the simplest form using elementary row operations.
fn reduce(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let rows = mat.len();
    for idx in 0..rows {
        mat = reduce_forward(mat, idx);
    }
    for idx in (0..rows).rev() {
        mat = reduce_backward(mat, idx);
    }
    mat
}

fn reduce_forward(mut mat: Vec<Vec<i32>>, row_idx: usize) -> Vec<Vec<i32>> {
    // Bring the row with the least leading zeros up.
    mat = bubble_up(mat, row_idx);

    let cols = mat[row_idx].len();
    let start_col_idx = count_leading_zero(&mat[row_idx]);
    if start_col_idx >= cols {
        return mat;
    }

    let divisor = gcd(&mat[row_idx][start_col_idx..]);
    for num in mat[row_idx].iter_mut() {
        (*num) /= divisor;
    }

    for i in (row_idx + 1)..mat.len() {
        if mat[i][start_col_idx] == 0 {
            // Already reduced.
            continue;
        }
        mat = reduce_wrt(mat, row_idx, i, start_col_idx);
    }
    mat
}

fn reduce_backward(mut mat: Vec<Vec<i32>>, row_idx: usize) -> Vec<Vec<i32>> {
    let cols = mat[row_idx].len();
    let start_col_idx = count_leading_zero(&mat[row_idx]);
    if start_col_idx >= cols {
        return mat;
    }

    for i in (0..row_idx).rev() {
        if mat[i][start_col_idx] == 0 {
            // Already reduced.
            continue;
        }
        mat = reduce_wrt(mat, row_idx, i, start_col_idx);
    }
    
    let divisor = gcd(&mat[row_idx][start_col_idx..]);
    for num in mat[row_idx].iter_mut() {
        (*num) /= divisor;
    }
    mat
}

fn reduce_wrt(mut mat: Vec<Vec<i32>>,
                  upper: usize, lower: usize,
                  start_col_idx: usize) -> Vec<Vec<i32>> {
    let lcm =
        num::integer::lcm(mat[upper][start_col_idx], mat[lower][start_col_idx]);
    let lower_multiplier = lcm / mat[lower][start_col_idx];
    let upper_multiplier = lcm / mat[upper][start_col_idx];

    let mut temp: Vec<i32> = Vec::new();
    for (lower_num, upper_num) in mat[lower].iter().zip(mat[upper].iter()) {
        let num =
            (*lower_num) * lower_multiplier - (*upper_num) * upper_multiplier;
        temp.push(num);
    }

    for (dst, src) in mat[lower].iter_mut().zip(temp.iter()) {
        *dst = *src;
    }
    mat
}

fn bubble_up(mut mat: Vec<Vec<i32>>, start_row_idx: usize) -> Vec<Vec<i32>> {
    if start_row_idx >= mat.len() {
        return mat;
    }
    let mut candidate = (0, std::usize::MAX);
    for (idx_offset, row) in mat.iter().skip(start_row_idx).enumerate() {
        let count = count_leading_zero(row);
        if count < candidate.1 {
            candidate = (start_row_idx + idx_offset, count);
        }
    }
    if candidate.0 != start_row_idx {
        swap_rows(&mut mat, candidate.0, start_row_idx);
    }
    mat
}

fn swap_rows(mat: &mut Vec<Vec<i32>>, i: usize, j: usize) {
    unsafe {
        let pa: *mut Vec<i32> = &mut mat[i];
        let pb: *mut Vec<i32> = &mut mat[j];
        std::ptr::swap(pa, pb);
    }
}

fn count_leading_zero(array: &Vec<i32>) -> usize {
    array.iter().take_while(|x| **x == 0).count()
}

fn gcd(array: &[i32]) -> i32 {
    let mut res = array[0];
    for num in array.iter() {
        if *num == 0 {
            continue;
        }
        res = num::integer::gcd(res, *num);
    }
    if array[0] > 0 {
        res
    } else {
        -res
    }
}

fn lcm(array: &[i32]) -> i32 {
    let mut res = array[0];
    for num in array.iter() {
        if *num == 0 {
            continue;
        }
        res = num::integer::lcm(res, *num);
    }
    res
}

/******************************************************************************/
/************************************ Parser **********************************/
/******************************************************************************/

fn parse(element: &str) -> HashMap<String, i32> {
    parse_recursive(element).0
}

fn parse_recursive(element: &str) -> (HashMap<String, i32>, usize) {
    let mut res: HashMap<String, i32> = HashMap::new();

    let mut num_literal = 0;
    let mut elem_literal: Option<String> = None;
    let mut cluster_literal: Option<HashMap<String, i32>> = None;

    let mut idx = 0;
    while idx < element.len() {
        let ch = element.chars().nth(idx).unwrap();
        idx += 1;

        if ch >= '0' && ch <= '9' {
            num_literal = 10 * num_literal + (ch as i32 - '0' as i32);
            continue;
        }

        if num_literal > 0 {
            update_res(&mut res, &mut elem_literal,
                       &mut cluster_literal, &mut num_literal);
        }

        if ((ch >= 'A' && ch <= 'Z') || ch == '(' || ch == ')' || ch == '$')
            && num_literal == 0 {
            update_res(&mut res, &mut elem_literal,
                       &mut cluster_literal, &mut 1);
        }

        match ch {
            '(' => {
                let (sub_res, len) = parse_recursive(&element[idx..]);
                cluster_literal = Some(sub_res);
                idx += len;
            },

            ')' | '$' => {
                return (res, idx);
            },

            _ => {
                elem_literal =
                    Some(elem_literal.map_or(
                        ch.to_string(), |elem| elem + &ch.to_string()));
            }
        }
    }
    (res, element.len())
}

fn update_res(res: &mut HashMap<String, i32>, elem: &mut Option<String>,
              cluster: &mut Option<HashMap<String, i32>>, num: &mut i32) {
    if !(elem.is_none() ^ cluster.is_none()) {
        // elem and cluster both are none or both are some.
        return;
    }

    if elem.is_some() {
        update_res_elem(res, elem.as_ref().unwrap(), *num);
    } else {
        update_res_cluster(res, cluster.as_ref().unwrap(), *num);
    }

    *num = 0;
    *elem = None;
    *cluster = None;
}

fn update_res_elem(res: &mut HashMap<String, i32>, elem: &str, num: i32) {
    let count = res.entry(String::from(elem)).or_insert(0);
    *count += num;
}

fn update_res_cluster(res: &mut HashMap<String, i32>,
                      cluster: &HashMap<String, i32>, num: i32) {
    for (elem, cnt) in cluster.iter() {
        let count = res.entry(String::from(elem)).or_insert(0);
        *count += num * cnt;
    }
}

/******************************************************************************/
/************************************* Entry **********************************/
/******************************************************************************/

fn equalize(lhs: &Vec<&str>, rhs: &Vec<&str>) -> Vec<String> {
    let mut all_elems: HashSet<String> = HashSet::new();
    let mut lhs_res: Vec<HashMap<String, i32>> = Vec::new();
    let mut rhs_res: Vec<HashMap<String, i32>> = Vec::new();
    for matter in lhs.iter() {
        let res = parse(matter);
        for elem in res.keys() {
            all_elems.insert(String::from(elem));
        }
        lhs_res.push(res);
    }
    for matter in rhs.iter() {
        let res = parse(matter);
        for elem in res.keys() {
            all_elems.insert(String::from(elem));
        }
        rhs_res.push(res);
    }

    let bases = nullspace(make_matrix(&all_elems, &lhs_res, &rhs_res));

    let mut res: Vec<String> = Vec::new();
    for basis in bases.iter() {
        res.push(compose_equation(basis, lhs, rhs));
    }
    res
}

fn compose_equation(basis: &Vec<i32>,
                    lhs: &Vec<&str>,
                    rhs: &Vec<&str>) -> String {
    let mut equation = String::new();
    for (idx, matter) in lhs.iter().enumerate() {
        equation += &format!("{} {} ", basis[idx], drop_tail(matter));
        if idx < lhs.len() - 1 {
            equation += "+ ";
        } else {
            equation += "== ";
        }
    }
    for (idx, matter) in rhs.iter().enumerate() {
        let real_idx = idx + lhs.len();
        equation += &format!("{} {} ", basis[real_idx], drop_tail(matter));
        if idx < rhs.len() - 1 {
            equation += "+ ";
        }
    }
    equation
}

fn compose_reaction(lhs: &Vec<&str>, rhs: &Vec<&str>) -> String {
    let mut equation = String::new();
    for (idx, matter) in lhs.iter().enumerate() {
        equation += &format!("{} ", drop_tail(matter));
        if idx < lhs.len() - 1 {
            equation += "+ ";
        } else {
            equation += "-> ";
        }
    }
    for (idx, matter) in rhs.iter().enumerate() {
        equation += &format!("{} ", drop_tail(matter));
        if idx < rhs.len() - 1 {
            equation += "+ ";
        }
    }
    equation
}

fn make_matrix(all_elems: &HashSet<String>,
               lhs_res: &Vec<HashMap<String, i32>>,
               rhs_res: &Vec<HashMap<String, i32>>) -> Vec<Vec<i32>> {
    let mut mat: Vec<Vec<i32>> = Vec::new();
    for elem in all_elems.iter() {
        let mut row: Vec<i32> = Vec::new();
        for matter in lhs_res.iter() {
            row.push(*matter.get(elem).unwrap_or(&0));
        }
        for matter in rhs_res.iter() {
            row.push(-matter.get(elem).unwrap_or(&0));
        }
        mat.push(row);
    }
    mat
}

fn drop_tail<'a>(s: &'a str) -> &'a str {
    &s[0..s.len() - 1]
}

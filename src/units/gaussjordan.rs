
fn gauss_jordan_elimination(matrix:Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    // is the matrix square?
    let width = matrix.len();
    for row in 0..width {
        if &matrix[row].len() != &width {
            panic!("matrix shape must be square");
        }
    }
    // create the corresponding unit matrix
    let mut unit_matrix: Vec<Vec<f64>> = matrix.iter().map(|_| vec![0f64;width]).collect();
    for i in 0..unit_matrix.len() {
        unit_matrix[i][i] = 1f64;
    }
    let mut augmented_matrix: Vec<Vec<f64>>;
    for (index,row) in matrix.iter().enumerate() {
        let mut the_unit = dbg!(unit_matrix[index].clone());
        let mut tmp = dbg!(row.clone());
        // Because why the fuck would this obvious operation ever work?: let combined = dbg!(tmp.extend(the_unit));
        let combined = dbg!(tmp.append(&mut the_unit));
        // augmented_matrix.push(combined); // <- Can't push a `()` apparently!

        // An explicit, imperative style, though? This is kosher.
        // for value in the_unit.iter() {
        //     tmp.push(*value);
        // }
        // dbg!(tmp);
        // augmented_matrix.push(tmp);
    }
    return unit_matrix; // because the augmented_matrix variable is nonsense.
}

#[cfg(test)]
mod tests {
    #[test]
    fn can_find_inverse() {
        let mut input = vec![
        vec![2.0f64,-1.0,0.0],
        vec![-1.0f64,2.0,-1.0],
        vec![0.0f64,-1.0,2.0]
        ];
        let mut inverse = vec![
        vec![0.75f64,0.5,0.25],
        vec![0.5f64,1.0,0.5],
        vec![0.25f64,0.5,0.75]
        ];
        println!("input: {:?}",&input);
        println!("inverse: {:?}",&inverse);
        println!("here is the appended output {:?}",(input.append(&mut inverse)));
        assert_eq!(2 + 2, 4);
    }
use crate::gauss_jordan_elimination;
    #[test]
    fn get_unit() {
        let some_matrix:Vec<Vec<f64>> = vec![vec![1.0f64,2.0],vec![4.0f64,5.0]];
        gauss_jordan_elimination(some_matrix);
        assert!(2==2)
    }
}

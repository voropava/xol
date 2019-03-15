use std::cmp;

// https://neerc.ifmo.ru/wiki/index.php?title=%D0%97%D0%B0%D0%B4%D0%B0%D1%87%D0%B0_%D0%BE_%D1%80%D0%B5%D0%B4%D0%B0%D0%BA%D1%86%D0%B8%D0%BE%D0%BD%D0%BD%D0%BE%D0%BC_%D1%80%D0%B0%D1%81%D1%81%D1%82%D0%BE%D1%8F%D0%BD%D0%B8%D0%B8,_%D0%B0%D0%BB%D0%B3%D0%BE%D1%80%D0%B8%D1%82%D0%BC_%D0%92%D0%B0%D0%B3%D0%BD%D0%B5%D1%80%D0%B0-%D0%A4%D0%B8%D1%88%D0%B5%D1%80%D0%B0
fn main() {
    let s1 = String::from("Hello");
    let s2 = String::from("wor");

    println!("Hello, wor = {}", levenstein(s1, s2));
}


fn levenstein(s1: String, s2: String) -> u32 {
    let mut vec = vec![vec![0, s2.len()]; s1.len()];

    let insert_cost: usize = 1;
    let delete_cost: usize = 1;
    let replace_cost: usize = 1;

    vec[0][0] = 0;

    for j in 1..s1.len() {
        vec[0][j] = vec[0][j - 1] + insert_cost;
    }

    let slice1 = s1.as_slice();
    let slice2 = s1.as_slice();

    for i in 1..s2.len() {
        vec[i][0] = vec[i - 1][0] + delete_cost;

        for j in 1..s1.len() {
            if slice1[j] != slice2[j] {
                let c1 = cmp::min(vec[i -1][j] + delete_cost, vec[i][j - 1] + insert_cost);

                vec[i][j] = cmp::min(c1, vec[i -1][j - 1] + replace_cost);
            } else {
                vec[i][j] = vec[i -1 ][j - 1];
            }
        }
    }

    return vec[s2.len()][s1.len()];
}

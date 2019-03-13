use std::cmp;

fn main() {
    let s1 = String::from("Hello");
    let s2 = String::from("wor");

    let clos = |len: u32| s1.len();


    println!("Hello, wor = {}", levenstein(s1, s2, clos));
}


fn levenstein(s1: String, s2: String, clos: &Fn) -> u32 {
    const M: u32 =

    const N: u32 = |len: u32| s2.len();

    let mut arr = [[0, N]; M];

    let insert_cost = 1u32;
    let delete_cost = 1u32;
    let replace_cost = 1u32;

    arr[0][0] = 0;

    for j in 1..s1.len() {
        arr[0][j] = arr[0][j - 1] + insert_cost;
    }

    for i in 1..s2.len() {
        arr[i][0] = arr[i - 1][0] + delete_cost;

        for j in 1..s1.len() {
            if s1.as_slice()[j] != s2.as_slice()[j] {
                let c1 = cmp::min(arr[i -1][j] + delete_cost, arr[i][j - 1] + insert_cost);

                arr[i][j] = cmp::min(c1, arr[i -1][j - 1] + replace_cost);
            } else {
                arr[i][j] = arr[i -1 ][j - 1];
            }
        }
    }

    return arr[s2.len()][s1.len()];
}

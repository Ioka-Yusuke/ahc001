use proconio::input;

fn main() {
    // input
    input! {
        n: usize,
        _xyz: [[i32; 3]; n],
    }
    
    // 二次元配列の作り方
    let mut ans = vec![[0; 4]; n];

    // 計算部分
    for i in 0..n{
        ans[i][0] = _xyz[i as usize][0 as usize];
        ans[i][1] = _xyz[i as usize][1 as usize];
        ans[i][2] = _xyz[i as usize][0 as usize] + 1;
        ans[i][3] = _xyz[i as usize][1 as usize] + 1;
    }

    // output
    for [a, b, c, d] in ans {
        println!("{} {} {} {}", a, b, c, d);
    }

    
}
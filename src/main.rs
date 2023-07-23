use proconio::input;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();
    
    // input
    input! {
        n: usize,
        xyz: [[i32; 3]; n],
    }
    
    // 二次元配列の作り方
    let mut ans = vec![[0; 4]; n];

    // 計算部分
    let mut flag_map = vec![[0; 10001]; 10001];
    for i in 0..n{
        ans[i][0] = (xyz[i as usize][0 as usize]) as usize;
        ans[i][1] = (xyz[i as usize][1 as usize]) as usize;
        ans[i][2] = (xyz[i as usize][0 as usize] + 1) as usize;
        ans[i][3] = (xyz[i as usize][1 as usize] + 1) as usize;
        
        for p in ans[i][0]..ans[i][2]+1 {
            for q in ans[i][1]..ans[i][3]+1 {
                flag_map[p as usize][q as usize] = i;
            }
        }
    }

    // 山登り
    const MIN_LENGH: usize = 0;
    const MAX_LENGH: usize = 10001;
  	const STEP: usize = 50;
    
    let end_time = Instant::now();
    let elapsed = end_time.duration_since(start_time);
    let elapsed_secs = elapsed.as_secs();
    let elapsed_nanos = elapsed.subsec_nanos();

    let total_secs = elapsed_secs as f64 + f64::from(elapsed_nanos) / 1_000_000_000.0;

    while total_secs < 4.5 {
      	// x+
        for i in 0..n {
            // let mut flag = 0;
            if MIN_LENGH <= ans[i][2]+STEP && ans[i][2]+STEP < MAX_LENGH {
              	let mut flag = 0;
              	for p in ans[i][0]..ans[i][2]+STEP+1 {
                  for q in ans[i][1]..ans[i][3]+1 {
                    if flag == 0 && (ans[i][2] < p || ans[i][3] < q) {
                      flag = flag_map[p as usize][q as usize];
                    }
                  }
              	}
              
                if flag == 0 {
                    flag_map[(ans[i][2]+STEP) as usize][(ans[i][3]) as usize] = i+1;
                    ans[i][2] += STEP;
    
                    // flagの管理
                    for p in ans[i][0]..ans[i][2]+1 {
                        for q in ans[i][1]..ans[i][3]+1 {
                            flag_map[p as usize][q as usize] = i+1;
                        }
                    }
                } 
            }
        }
      	
      	// y+
      	for i in 0..n {
            // let mut flag = 0;
            if MIN_LENGH <= ans[i][3]+STEP && ans[i][3]+STEP < MAX_LENGH {
              	let mut flag = 0;
              	for p in ans[i][0]..ans[i][2]+1 {
                  for q in ans[i][1]..ans[i][3]+STEP+1 {
                    if flag == 0 && (ans[i][2] < p || ans[i][3] < q) {
                      flag = flag_map[p as usize][q as usize];
                    }
                  }
              	}
              
                if flag == 0 {
                    flag_map[(ans[i][2]) as usize][(ans[i][3]+STEP) as usize] = i+1;
                    ans[i][3] += STEP;
    
                    // flagの管理
                    for p in ans[i][0]..ans[i][2]+1 {
                        for q in ans[i][1]..ans[i][3]+1 {
                            flag_map[p as usize][q as usize] = i+1;
                        }
                    }
                }
            }
        }
      	
      	// x-
        for i in 0..n {
            // let mut flag = 0;
            if MIN_LENGH <= ans[i][0]-STEP && ans[i][0]-STEP < MAX_LENGH {
              	let mut flag = 0;
              	for p in ans[i][0]-STEP..ans[i][2]+1 {
                  for q in ans[i][1]..ans[i][3]+1 {
                    if flag == 0 && (ans[i][0] > p || ans[i][1] > q) {
                      flag = flag_map[p as usize][q as usize];
                    }
                  }
              	}
              
                if flag == 0 {
                    flag_map[(ans[i][0]-STEP) as usize][(ans[i][1]) as usize] = i+1;
                    ans[i][0] -= STEP;
    
                    // flagの管理
                    for p in ans[i][0]..ans[i][2]+1 {
                        for q in ans[i][1]..ans[i][3]+1 {
                            flag_map[p as usize][q as usize] = i+1;
                        }
                    }
                }
            }
        }
      	
      	// y-
      	for i in 0..n {
            // let mut flag = 0;
            if MIN_LENGH <= ans[i][1]-STEP && ans[i][1]-STEP < MAX_LENGH {
              	let mut flag = 0;
              	for p in ans[i][0]..ans[i][2]+1 {
                  for q in ans[i][1]-STEP..ans[i][3]+1 {
                    if flag == 0 && (ans[i][0] > p || ans[i][1] > q) {
                      flag = flag_map[p as usize][q as usize];
                    }
                  }
              	}
              
                if flag == 0 {
                    flag_map[(ans[i][0]) as usize][(ans[i][1]-STEP) as usize] = i+1;
                    ans[i][1] -= STEP;
    
                    // flagの管理
                    for p in ans[i][0]..ans[i][2]+1 {
                        for q in ans[i][1]..ans[i][3]+1 {
                            flag_map[p as usize][q as usize] = i+1;
                        }
                    }
                }
            }
        }
      	
        let end_time = Instant::now();
        let elapsed = end_time.duration_since(start_time);
        let elapsed_secs = elapsed.as_secs();
        let elapsed_nanos = elapsed.subsec_nanos();

        let total_secs = elapsed_secs as f64 + f64::from(elapsed_nanos) / 1_000_000_000.0;
    }
    
    // output
    for i in 0..n {
        println!("{} {} {} {}", ans[i as usize][0 as usize], ans[i as usize][1 as usize], ans[i as usize][2 as usize], ans[i as usize][3 as usize]);
    }
}
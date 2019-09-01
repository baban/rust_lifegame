use std::io::{stdout, Write, BufWriter};
use std::{thread, time};

fn main() {
    let ten_millis = time::Duration::from_millis(200);
    let mut table = set_galaxy_table();
    // table 定義
    for i in 1..100 {
        // table 表示
        print_table(i, &table);
        // 世代計算
        table = calc(&table);
        thread::sleep(ten_millis);
    }
}

fn set_beacon_table() -> Vec<i32> {
    let mut table = vec![0; 25*25];
    let data = vec![
        vec![0,0,0,0,0,0],
        vec![0,1,1,0,0,0],
        vec![0,1,1,0,0,0],
        vec![0,0,0,1,1,0],
        vec![0,0,0,1,1,0]
    ];
    for i in 0..5 {
        for j in 0..5 {
            table[(j * 25 + i) as usize] = data[i][j];
        }
    }

    return table;
}

fn set_galaxy_table() -> Vec<i32> {
    let mut table = vec![0; 25*25];
    let data = vec![
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,1,1,0,1,1,1,1,1,1,0,0,0],
        vec![0,0,0,1,1,0,1,1,1,1,1,1,0,0,0],
        vec![0,0,0,1,1,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,1,1,0,0,0,0,0,1,1,0,0,0],
        vec![0,0,0,1,1,0,0,0,0,0,1,1,0,0,0],
        vec![0,0,0,1,1,0,0,0,0,0,1,1,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,1,1,0,0,0],
        vec![0,0,0,1,1,1,1,1,1,0,1,1,0,0,0],
        vec![0,0,0,1,1,1,1,1,1,0,1,1,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    ];
    for i in 0..14 {
        for j in 0..13 {
            table[(j * 25 + i) as usize] = data[i][j];
        }
    }

    return table;
}

fn print_table(generation: i32, table: &Vec<i32>) {
    let mut s = format!("generation: {}\n", generation);
    for i in 0..25 {
        for j in 0..25 {
            if table[j * 25 + i] == 1 {
                s += "* ";
            } else {
                s += "- ";
            }
        }
        s += "\n";
    }
    println!("\x1B[2J{}", s);
    // println!("{}", s);
}

fn calc(table: &Vec<i32>) -> Vec<i32> {
    let mut table2 = vec![0; 25*25];
    for i in 0..25 {
        for j in 0..25 {
            let c = count_round(&table, j, i);
            let state = cell_state(table[(j * 25 + i) as usize], c);
            table2[(j * 25 + i) as usize] = state;
        }
        // println!("");
    }
    return table2;
}

fn count_round(table: &Vec<i32>, w: i32, h: i32) -> i32 {
    let directions = vec![
        (-25 - 1), -25, (-25 +1),
        -1, 1,
        (25 - 1), 25, (25 +1),
    ];
    let start = w * 25 + h;
    let mut c = 0;
    for d in directions {
        let p = start + d;
        if(0 <= p && p < 25 * 25 && table[p as usize] == 1) {
            c += 1;
        }
    }
    // print!("{} ",c);
    return c;
}

fn cell_state(state: i32, c: i32) -> i32 {
    //    誕生
    //    死んでいるセルに隣接する生きたセルがちょうど3つあれば、次の世代が誕生する。
    //生存
    //    生きているセルに隣接する生きたセルが2つか3つならば、次の世代でも生存する。
    //過疎
    //    生きているセルに隣接する生きたセルが1つ以下ならば、過疎により死滅する。
    //過密
    //    生きているセルに隣接する生きたセルが4つ以上ならば、過密により死滅する。
    return match state {
        0 => match c {
            3 => 1,
            _ => 0
        },
        _ => match c {
            2 => 1,
            3 => 1,
            _ => 0
        }
    };
}

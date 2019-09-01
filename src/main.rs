use std::io::{stdout, Write, BufWriter};
use std::{thread, time};

fn main() {
    //println!("Hello, world!");
    let ten_millis = time::Duration::from_millis(100);
    let mut table = set_init_table();
    // table 定義
    for i in 1..2 {
        // table 表示
        print_table(i, &table);
        // 世代計算
        table = calc(&table);
        thread::sleep(ten_millis);
    }
}

fn set_init_table() -> Vec<i32> {
    let mut table = vec![0; 25*25];
    let data = init_data();
    for i in 0..5 {
        for j in 0..5 {
            table[(j * 25 + i) as usize] = data[i][j];
        }
    }

    return table;
}

fn init_data() -> Vec<Vec<i32>> {
    let data = vec![
        vec![0,0,0,0,0,0],
        vec![0,1,1,0,0,0],
        vec![0,1,1,0,0,0],
        vec![0,0,0,1,1,0],
        vec![0,0,0,1,1,0]
    ];
    return data;
}

fn set_grider(table: Vec<i32>) -> Vec<i32> {

    return table;
}

fn print_table(generation: i32, table: &Vec<i32>) {
    //print!("\x1B[2J");
    println!("generation: {}", generation);
    for i in 0..25 {
        for j in 0..25 {
            if table[j * 25 + i] == 1 {
                print!("{} ", "*");

            } else {
                print!("{} ", "-");
            }

        }
        println!("");
    }
}

fn calc(table: &Vec<i32>) -> Vec<i32> {
    let mut table2 = vec![0; 25*25];
    for i in 0..25 {
        for j in 0..25 {
            let c = count_round(&table, i, j);
            let state = cell_state(table[(j * 25 + i) as usize], c);
            table2[(j * 25 + i) as usize] = state;
        }
    }
    return table2;
}

fn count_round(table: &Vec<i32>, w: i32, h: i32) -> i32 {
    for i in 0..7 {

    }
    return 0;
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

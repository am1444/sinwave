use std::io;
use std::{thread, time::Duration};
pub mod degree;

fn repeat_tab_print(disp:Vec<bool>) {
    for current in disp {
        if current {
            print!("#");
        } else {
            print!(" ");
        }
    }
    print!("\n");
}

fn main() {
    /*let rep = 50;
    let dist:usize = 50;
    let termwidth:i32 = 255;*/
    
    let mut buffer:String = String::new();

    println!("\nRep?:");
    io::stdin().read_line(&mut buffer).expect("Error reading string");
    let rep:i64 = buffer.trim().parse::<i64>().unwrap();
    buffer = String::new();
    
    println!("\nDist?:");
    io::stdin().read_line(&mut buffer).expect("Error reading string");
    let dist:i64 = buffer.trim().parse::<i64>().unwrap();
    buffer = String::new();

    println!("\nWidth?:");
    io::stdin().read_line(&mut buffer).expect("Error reading string");
    let termwidth = buffer.trim().parse::<i64>().unwrap();
    buffer = String::new();

    println!("\nWait?:");
    io::stdin().read_line(&mut buffer).expect("Error reading string");
    let wait:i32 = buffer.trim().parse::<i32>().unwrap();

    let termdisp = termwidth / 2;
    let mut current:usize = 1;
    loop {
        let mut disp:Vec<bool> = vec![false;termwidth as usize];
        for count in 1..=rep {
            let x = degree::sind64((count * dist + current as i64) as f64);
            disp[((termdisp as f64 * x).ceil()-1.0 + termdisp as f64) as usize] = true;
        }
        /*println!("{:?}",disp);*/
        thread::sleep(Duration::from_millis(wait.try_into().unwrap()));
        repeat_tab_print(disp);
        current = current + 1;
    }
}

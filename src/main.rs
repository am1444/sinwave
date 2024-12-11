//Sinwave 1.2
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

fn isin(clargs:Vec<String>,target:String) -> i32 {
    for item in 1..clargs.len() {
        if clargs[item] == target { return item.try_into().unwrap(); }
    }
    return 0;
}

fn getin(clargs:Vec<String>,target:String) -> Result<String,bool> {
    let targ_index = isin(clargs.clone(),target.clone());
    if targ_index == 0 { return Err(false); } // Target is not in clargs
    if targ_index == clargs.len().try_into().unwrap() { return Err(true); } //Target is last item in clargs
    return Ok(clargs[targ_index as usize + 1_usize].clone());
}

fn main() {
    let mut buffer:String = String::new();
    let clargs:Vec<String> = std::env::args().collect::<Vec<String>>();

    match getin(clargs.clone(),"-r".to_string()){
        Err(_) => {println!("\nRep?:");
        io::stdin().read_line(&mut buffer).expect("Error reading string");},
        Ok(inp) => buffer = inp,
    }
    let rep:i64 = buffer.trim().parse::<i64>().unwrap();
    buffer = String::new();

    match getin(clargs.clone(),"-d".to_string()){
        Err(_) => {println!("\nDist?:");
        io::stdin().read_line(&mut buffer).expect("Error reading string");},
        Ok(inp) => buffer = inp,
    }
    let dist:i64 = buffer.trim().parse::<i64>().unwrap();
    buffer = String::new();
    
    match getin(clargs.clone(),"-wd".to_string()){
        Err(_) => {println!("\nWidth?:");
        io::stdin().read_line(&mut buffer).expect("Error reading string");},
        Ok(inp) => buffer = inp,
    }
    let termwidth:i64 = buffer.trim().parse::<i64>().unwrap();
    buffer = String::new();
    
    match getin(clargs.clone(),"-wt".to_string()) {
        Err(_) => {println!("\nWait?:");
        io::stdin().read_line(&mut buffer).expect("Error reading string");},
        Ok(inp) => buffer = inp,
    }
    let wait:i32 = buffer.trim().parse::<i32>().unwrap();

    let termdisp = termwidth / 2;
    let mut current:usize = 1;
    let mut current_sin = 0_f64;
    loop {
        let mut disp:Vec<bool> = vec![false;termwidth as usize];
        for count in 1..=rep {
            current_sin = degree::sind64((count * dist + current as i64) as f64);
            disp[((termdisp as f64 * current_sin).ceil()-1.0 + termdisp as f64) as usize] = true;
        }
        /*println!("{:?}",disp);*/
        thread::sleep(Duration::from_millis(wait.try_into().unwrap()));
        repeat_tab_print(disp);
        if current_sin==0_f64 {current=0_usize;}
        current = current + 1;
    }
}


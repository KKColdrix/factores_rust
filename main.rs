fn main() {
    let user_arg = std::env::args().nth(1)
        .expect("Write a number in args")
        .parse::<i32>()
        .unwrap();
    
    let mut even_nums = Vec::new();
    

    for i in 1..=user_arg {
        if user_arg % i == 0 {
            even_nums.push(i);
        }
    }

    println!("Printing factor numbers of {}...",user_arg);

    for i in 1..even_nums.len()  {
        if i % 5 == 0  {
            println!(" {}", even_nums[i]);
        }
        else{   
            print!(" {}", even_nums[i]);
        }

    }
    
}

/* use clap::Parser;
use indicatif;

#[derive(Parser)]
struct Cli {
    arg1 : String,
    arg2 : String
} */
    //using std function
    //let test_1 = std::env::args().nth(1).expect("insert an argument");
    //let test_2 = std::env::args().nth(2).expect("insert a second argument");    
    
    /* let my_args = Cli::parse();

    if my_args.arg1 == "hola" {
        println!("hola");
    }
    if my_args.arg2 == "adios" {
        println!("adios")
    }

    let pb = indicatif::ProgressBar::new(1000);
    for i in 0..1000 {
        //do_hard_work();
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done"); */


use std::io::{self, Write}; 

fn fibo_series(mut n: u64) {

    println!("\n== Fibonnaci Series till {n} ==");

    if n == 1 {
        println!("0");
        return;
    }
    let mut first: u64 = 0; 
    let mut second: u64 = 1; 
    print!("{first} {second} ");
    while n > 2 {
        let third: u64 = first + second; 
        print!("{third} ");
        first = second; 
        second = third; 
        n -= 1;  
    }

}

fn fibo_number(n: u64) -> u64 {
    if n <= 1 {
        0
    } else if n == 2 {
        1
    } else {
        fibo_number(n - 1) + fibo_number(n - 2) 
    }
}

fn main() {
    println!("==== Fibonnaci Number Program ===="); 

    loop {
        println!("\nChoose from the operations: 
        1. Generate Fibonnaci series
        2. Nth Fibonnaci Number
        3. Exit");

        let mut choice = String::new(); 
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line"); 

        let choice = choice.trim(); 

        if choice == "1" {

            print!("Enter a positive number: ");
            io::stdout().flush().expect("Failed to flush");

            let mut n = String::new(); 
            io::stdin()
                .read_line(&mut n)
                .expect("Failed to read line"); 

            let n: u64 = n.trim().parse().expect("Enter a positive number");
            fibo_series(n); 

        } else if choice == "2" {
            
            print!("Enter a positive number: "); 
            io::stdout().flush().expect("Failed to flush"); 

            let mut n = String::new(); 
            io::stdin()
                .read_line(&mut n)
                .expect("Failed to read line"); 

            let n: u64 = n.trim().parse().expect("Enter a positive number"); 
            println!("{n}th Fibonnaci Number is {}", fibo_number(n)); 

        } else if choice == "3"{
            println!("Good Bye!!");
            break; 
        } else {
            println!("Invalid Input.");
        }
    }
}
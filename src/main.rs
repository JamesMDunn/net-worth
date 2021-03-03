extern crate clap;
use std::ops::Sub;

use clap::{App, Arg, SubCommand};


struct Compound_Formula {
    principle: f32,
    compound_time: f32,
    annual_rate: f32,
    time: f32
}

impl Default for Compound_Formula {
    fn default() -> Self {
        Compound_Formula {
            principle: 10000.00,
            compound_time: 1.0,
            annual_rate: 0.80,
            time: 10.0,
        }
    }
}


impl Compound_Formula {
    fn calculate_compound(&self) -> f32 {
        self.principle * (f32::powf(1.0 + (self.annual_rate / self.time), self.compound_time * self.time))
    }
}

fn main() {
    let matches = App::new("Net worth")
        .version("0.1")
        .author("James Dunn")
        .about("Does awesome things")
        .subcommand(
            SubCommand::with_name("salary").about("Stuff about your salary").arg(
                Arg::with_name("income")
                .help("your income")
                .required(true),
            ),
        )
        .subcommand(
            SubCommand::with_name("compound").about("compount interest calculator").arg(
                Arg::with_name("principle")
                .help("the portfolio amount")
                .required(true),
            ),
        )
        .get_matches();

    match matches.subcommand() {
        ("salary", Some(salary_matches)) => {
            let income_int: i32 = salary_matches.value_of("income").unwrap().to_string().parse().expect("testing");
            println!("your weekly salary is {}", income_int / 52);
            println!("Your bi-weekly salary is {}", income_int / 26);
            println!("your monthly salary is {}", income_int / 12);
        },
        ("compound", Some(compound_matches)) => {
            let principle: f32 = compound_matches.value_of("principle").unwrap().to_string().parse().expect("this to be a float");
            let return_investment = Compound_Formula { principle,..Default::default()}.calculate_compound();
            println!("your return on investment is {}", return_investment)
        },
        _ => unreachable!(), // Assuming you've listed all direct children above, this is unreachable
    }
}



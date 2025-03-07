extern crate clap;

use core::f32;

use clap::{App, Arg, SubCommand};

const NAN_ERROR: &str = "Expected a number";

struct CompoundFormula {
    principle: f32,
    compound_time: f32,
    annual_rate: f32,
    time: f32
}

impl Default for CompoundFormula {
    fn default() -> Self {
        CompoundFormula {
            principle: 10000.00,
            compound_time: 1.0,
            annual_rate: 0.80,
            time: 10.0,
        }
    }
}

impl CompoundFormula {
    fn calculate_compound(&self) -> f32 {
        self.principle * (f32::powf(1.0 + (self.annual_rate / self.compound_time), self.compound_time * self.time))
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
            SubCommand::with_name("compound").about("Compound interest calculator").arg(
                Arg::with_name("principle")
                .help("the portfolio amount")
                .required(true),
            )
            .arg(Arg::with_name("annual_rate")
                .help("expected annual rate")
                .required(false),
            )
            .arg(Arg::with_name("time")
                .help("years invested")
                .required(false),
            ),
        ).subcommand(
            SubCommand::with_name("average_share_price").about("Average share price").arg(
                Arg::with_name("price")
                .help("price of your shares when purchased")
                .required(true)
                .multiple(true)
            ).arg(Arg::with_name("quanity")
                .help("number of shares")
                .required(true)
                .multiple(true)
                .last(true)
            )     
        )
        .get_matches();

    match matches.subcommand() {
        ("salary", Some(salary_matches)) => {
            let income_int: i32 = salary_matches.value_of("income").unwrap().to_string().parse().expect(NAN_ERROR);

            println!("your weekly salary is {}", income_int / 52);
            println!("Your bi-weekly salary is {}", income_int / 26);
            println!("your monthly salary is {}", income_int / 12);
        },
        ("compound", Some(compound_matches)) => {
            let mut formula = CompoundFormula { ..Default::default() };
            let principle: f32 = compound_matches.value_of("principle").unwrap().to_string().parse().expect(NAN_ERROR);
            if let Some(annual_rate) = compound_matches.value_of("annual_rate") {
                formula.annual_rate = annual_rate.to_string().parse().expect(NAN_ERROR);
            }
            if let Some(time) = compound_matches.value_of("time") {
                formula.time = time.to_string().parse().expect(NAN_ERROR);
            }
            formula.principle = principle;
            let return_investment = formula.calculate_compound();
            println!("your return on investment is {}", return_investment);
        },
        ("average_share_price", Some(average_price)) => {
            let prices: Vec<&str> = average_price.values_of("price").unwrap().collect();
            let quantities: Vec<&str> = average_price.values_of("quanity").unwrap().collect();
            if prices.len() != quantities.len() {
                println!("Mismatch between shares and prices")
            } else {
                let combined_totals: Vec<f32> = prices.iter().zip(quantities.iter()).map(|(&x, &j)| x.to_string().parse::<f32>().expect(NAN_ERROR) as f32 * j.to_string().parse::<f32>().expect(NAN_ERROR) as f32).collect::<Vec<f32>>();
                let totals: f32 = combined_totals.iter().sum();
                let quantities_sum: f32 = quantities.iter().map(|x| x.to_string().parse::<f32>().expect(NAN_ERROR)).sum();
                println!("\nTotal amount of money spent: ${}\nBreak even average: ${:?}", totals, totals / quantities_sum )
            }
        },
        _ => unreachable!(), // Assuming you've listed all direct children above, this is unreachable
    }
}

// let zipped = prices.iter().zip(quantities.iter());
// println!("zipped {:?}", zipped.collect::<Vec<(&&str, &&str)>>())

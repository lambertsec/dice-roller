use std::io;
use crate::dice_tools::Dice;

fn main() {

    let mut input = String::new();

    println!("dice roller");
    println!("How many dice do you need to roll?");
    match io::stdin().read_line(&mut input).unwrap() {
        _none => println!("You need to roll?") };

    let dice_number :i8 = input.trim().parse().unwrap();
    let mut dice_list: Vec<Dice> = Vec::new();

    for i in 0..dice_number {
        println!("how many side does dice {} have", i+1);
        let mut sides = String::new();
        match io::stdin().read_line(&mut sides).unwrap() { _none => println!("You need to enter a value?") };
        let side :i8 = sides.trim().parse().unwrap();
        &dice_list.push(Dice::new(side));
    }

    let mut outcomes = Vec::new();

    for die in dice_list{
        &outcomes.push(Dice::roll_dice(die));
    }

    let outcome_string = Dice::calculate_outcomes(&outcomes);



    let total:i8 = Dice::total(outcomes);

    println!(" {} dice were rolled. The outcome values are {}. The total value is: {}", dice_number, outcome_string, total);



}


pub mod dice_tools{
    use rand::Rng;

    pub struct Dice{
        sides: i8


    }

    impl Dice{
        pub fn new(sides:i8)-> Dice{
            Dice{sides}
        }
        pub fn roll_dice(die: Dice) ->i8 {
            rand::thread_rng().gen_range(1..=die.sides)

        }
        pub fn total(values:Vec<i8>) -> i8 {
            let mut total:i8 = 0;
            for value in values{
                total += value;
            }
            total
        }

        pub fn calculate_outcomes(outcomes:&Vec<i8>)-> String {
            let mut output = String::new();
            for item in outcomes {

                if !outcomes.last().eq(&Some(&item)) {
                    output += &item.to_string();
                    output += ", ";
                }else{
                    output += "and ";
                    output += &item.to_string();
                }

            }
            output
        }
    }

    mod dice_tests{
        use super::*;

        #[test]
        pub fn test_total(){
            let mut numbers:Vec<i8> = Vec::new();
            numbers.push(1);
            numbers.push(2);
            numbers.push(3);

            assert_eq!(Dice::total(numbers), 6);

        }

        #[test]
        pub fn test_calculate_outcomes(){
            let mut numbers:Vec<i8> = Vec::new();
            numbers.push(1);
            numbers.push(2);
            numbers.push(3);
            assert_eq!(Dice::calculate_outcomes(&numbers), "1, 2, and 3");
        }
    }

}
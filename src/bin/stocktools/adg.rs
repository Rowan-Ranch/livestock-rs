use anyhow::{Context, Result};
use clap::Parser;
use livestock_rs::calculators::growth::adg::calculate_adg;

#[derive(Parser, Debug)]
#[command(
    arg_required_else_help(true),
    about = "Calculate Average Daily Gain (ADG) for livestock.",
    long_about = "
        Calculate Average Daily Gain (ADG) for livestock.

        The Average Daily Gain (ADG) is a measure of the average weight gain of an animal per day.
        It is calculated by subtracting the initial weight from the final weight and dividing by the
        number of days in the measurement period.

        The formula is:

        ADG = (final_weight - initial_weight) / days

        where:

        - `ADG` is the average daily gain in the same unit as the weight.
        - `final_weight` is the ending weight of the animal (in kg or lbs).
        - `initial_weight` is the starting weight of the animal (in kg or lbs).
        - `days` is the number of days in the measurement period.

        # Example

        Calculate the ADG for an animal that starts at 100 kg and ends at 150 kg over 50 days:

        ```
        livestock adg -i 100 -f 150 -d 50
        ```

        The result will be `1.0`, which means the animal gained 1 kg per day.        
    "
)]
pub struct AdgSubcommand {
    #[arg(help = "Initial weight of livestock (in kg or lbs)", long, short = 'i')]
    initial_weight: f64,
    #[arg(help = "Final weight of livestock (in kg or lbs)", long, short = 'f')]
    final_weight: f64,
    #[arg(
        long,
        help = "The number of days in the measurement period.",
        short = 'd'
    )]
    days: usize,
}

impl AdgSubcommand {
    pub fn run(&self) -> Result<()> {
        let adg = calculate_adg(self.initial_weight, self.final_weight, self.days)
            .context("Failed to calculate ADG.")?;

        println!("Average Daily Gain (ADG): {:.2}", adg);
        Ok(())
    }
}

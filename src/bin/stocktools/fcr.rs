use anyhow::{Context, Result};
use clap::Parser;
use livestock_rs::calculators::feed::fcr::calculate_fcr;

#[derive(Parser, Debug)]
#[command(
    arg_required_else_help(true),
    about = "Calculate Feed Conversion Ratio (FCR) for livestock.",
    long_about = "
        Calculate Feed Conversion Ratio (FCR) for livestock.

        The Feed Conversion Ratio (FCR) is a measure of feed efficiency that indicates the amount of
        feed required to produce a unit of weight gain in livestock. It is calculated by dividing the
        amount of feed consumed by the weight gain of the animal.

        The formula is:

        FCR = feed_intake / weight_gain

        where:

        - `FCR` is the feed conversion ratio as a ratio of feed intake to weight gain.
        - `feed_intake` is the amount of feed consumed by the animal (in kg or lbs).
        - `weight_gain` is the weight gain of the animal (in kg or lbs).

        # Example

        Calculate the FCR for an animal that consumes 100 kg of feed and gains 150 kg:

        ```
        livestock fcr -i 100 -g 150
        ```

        The result will be `0.67`, which means the animal required 0.67 kg of feed per kg of weight gain.
    "
)]
pub struct FcrSubcommand {
    #[arg(help = "Amount of feed intake (in kg or lbs)", long, short = 'i')]
    feed_intake: f64,
    #[arg(help = "Weight gain of livestock (in kg or lbs)", long, short = 'g')]
    weight_gain: f64,
}

impl FcrSubcommand {
    pub fn run(&self) -> Result<()> {
        let fcr = calculate_fcr(self.feed_intake, self.weight_gain)
            .context("Failed to calculate FCR.")?;

        println!("Feed Conversion Ratio (FCR): {:.2}", fcr);
        Ok(())
    }
}

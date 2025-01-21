use anyhow::{anyhow, Context, Result};
use clap::Parser;
use livestock_rs::{calculators::feed::{efficiency::calculate_feed_efficiency, fcr::calculate_fcr}, types::LivestockType};

#[derive(Parser, Debug)]
#[command(
    arg_required_else_help(true),
    about = "Calculate Feed Efficiency for livestock.",
    long_about = "
        Calculate Feed Efficiency for livestock.

        The Feed Efficiency is a measure of how well livestock convert feed into body mass. It is
        calculated using the Feed Conversion Ratio (FCR) of the animal.

        The formula is:

        Feed Efficiency = 1 / FCR

        where:

        - `Feed Efficiency` is the efficiency of feed conversion.
        - `FCR` is the Feed Conversion Ratio of the animal.

        # Example

        Calculate the Feed Efficiency for an animal with a FCR of 2.0:

        ```
        stocktools feed-efficiency --fcr 2.0 --livestock-type Cattle
        ```

        The result will be `0.5`, which means the animal converted feed with an efficiency of 0.5. This is very good for cattle.      
    "
)]
pub struct FeedEfficiencySubcommand {
    #[arg(help = "Amount of feed intake (in kg or lbs)", long, short = 'i')]
    feed_intake: Option<f64>,
    #[arg(help = "Weight gain of livestock (in kg or lbs)", long, short = 'g')]
    weight_gain: Option<f64>,
    #[arg(help = "Feed efficiency ratio (FCR)", long)]
    fcr: Option<f64>,
    #[arg(
        long,
        help = "The type of livestock.",
        short = 't',
    )]
    livestock_type: LivestockType,
}

impl FeedEfficiencySubcommand {
    pub fn run(&self) -> Result<()> {
        // ensure that we either have feed intake and weight gain or FCR
        let fcr = match (self.feed_intake, self.weight_gain, self.fcr) {
            (Some(feed_intake), Some(weight_gain), None) => {
                calculate_fcr(feed_intake, weight_gain).context(
                    "Failed to calculate FCR, which is required if feed intake and weight gain are provided.
                    FCR is needed to calculate feed efficiency.",
                )?
            }
            (None, None, Some(fcr)) => fcr,
            _ => {
                return Err(anyhow!(
                    "Either feed intake and weight gain or FCR must be provided."
                ))
            }
        };

        let feed_efficiency = calculate_feed_efficiency(fcr, self.livestock_type.clone()).with_context(|| format!(
            "Failed to calculate feed efficiency with FCR: {} and livestock type: {:?}.",
            fcr, self.livestock_type
        ))?;
    
        println!(" ");
        println!("Feed Efficiency Rating: {:?}", feed_efficiency.rating);
        println!("FCR: {:.2}", feed_efficiency.value);
        println!("{:?} should aim for a FCR between {:.2} and {:.2}.", self.livestock_type, feed_efficiency.avg_min_fcr, feed_efficiency.avg_max_fcr);
        println!(" ");

        Ok(())
    }
}

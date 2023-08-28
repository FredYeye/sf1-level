// use plotters::prelude::*;
use sf::{Sf, stats::Id};

use crate::sf::character::StatGainMethod;

mod sf;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let help = concat!(
        "commands\n",
        "-minmax-all   | generate min-max stats for all characters / levels\n",
        "-minmax-start | generate min-max starting stats\n",
        "-target       | generate target stats for all characters / levels\n",
    );

    if args.len() > 1 {
        match &args[1] as &str {
            "-minmax-all" => {
                for id in Id::get_ids() {
                    let mut sf = Sf::init(id, 0);
                    sf.all_levels_min_max();
                    println!();
                }
            }

            "-minmax-start" => {
                for id in Id::get_ids() {
                    let mut sf = Sf::init(id, 0);
                    sf.starting_level_min_max();
                    println!();
                }
            }

            "-target" => {
                for id in Id::get_ids() {
                    let mut sf = Sf::init(id, 0);
                    sf.log_stats(&StatGainMethod::Target);
                    println!();
                }
            }

            _ => println!("{help}"),
        }
    } else {
        println!("{help}");
    }

    // let _ = test();
}

// fn test() -> Result<(), Box<dyn std::error::Error>> {
//     let attack = [ 6,  7,  9, 11, 11, 12, 13, 13, 13, 14, 14, 15, 15, 16, 17, 17, 19, 20, 21, 23];
//     let min    = [ 6,  7,  8, 10, 11, 11, 12, 13, 13, 13, 14, 14, 15, 15, 16, 17, 17, 19, 20, 21];
//     let max    = [ 6,  7, 10, 12, 12, 14, 15, 15, 15, 16, 16, 17, 17, 19, 20, 20, 22, 24, 25, 27];

//     let root = BitMapBackend::new("0.png", (800, 800)).into_drawing_area();
//     root.fill(&WHITE)?;
//     let mut chart = ChartBuilder::on(&root)
//         .caption("attack", ("sans-serif", 50).into_font())
//         .margin(10)
//         .x_label_area_size(30)
//         .y_label_area_size(30)
//         .build_cartesian_2d(1..20, 0..27)?;

//     chart.configure_mesh().draw()?;

//     chart
//         .draw_series(LineSeries::new(
//             attack.iter().enumerate().map(|(idx, x)| (idx as i32 + 1, *x as i32)),
//             RED.filled(),
//         ).point_size(2))?;

//     chart
//         .draw_series(attack.iter().enumerate().map(|f| {
//             ErrorBar::new_vertical(f.0 as i32 + 1, min[f.0], *f.1, max[f.0], BLUE.filled(), 10)
//         }))?;

//     chart
//         .configure_series_labels()
//         .background_style(&WHITE.mix(0.8))
//         .border_style(&BLACK)
//         .draw()?;

//     root.present()?;

//     Ok(())
// }

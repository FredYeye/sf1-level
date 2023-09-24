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

    // let mut sf = Sf::init(Id::Max, 0);
    // sf.promote_test(10);

    // let mut sf = Sf::init(Id::Max, 0);
    // sf.speed_test();

    // resist();
}

fn resist() {
    let spell_multiplier = [100, 75, 50, 125];
    let resist_chance = [37, 18, 0, 46];
    let resist = 0x838d;

    let mut fix = [0; 8];

    for x in 0..8 {
        fix[x] = (resist >> (x * 2)) & 0b11;
    }

    if fix[6] > 0 {
        println!("omg");
    }

    // println!("Blaze | Freeze | Bolt | Sleep/Desoul | Muddle | Slow |");
    println!("{:<6}| {:<7}| {:<5}| {:<13}| {:<7}| {:<4} | {:<2}| {:<5} |",
    spell_multiplier[fix[0]], spell_multiplier[fix[1]], spell_multiplier[fix[2]],
    resist_chance[fix[3]], resist_chance[fix[4]], resist_chance[fix[5]],
    fix[6], fix[7]);


    // CLASS_RESIST_SLEEP_AND_DESOUL: equ $3

    // ; enum ClassEntry_Resistances
    // CLASS_RESIST_BLAZE: equ $0
    // CLASS_RESIST_FREEZE: equ $1
    // CLASS_RESIST_BOLT: equ $2
    // CLASS_RESIST_SLEEP_AND_DESOUL: equ $3
    // CLASS_RESIST_MUDDLE: equ $4
    // CLASS_RESIST_SLOW: equ $5
    // CLASS_RESIST_6: equ $6
    // CLASS_RESIST_EVASION: equ $7
    
    // u16 res
    // 3 & 0b111
    // res ror 3
    // res ror 3
    // res & 0b11
    
    
    
    
    
    
    // sub_212FA (d2 = resist)
    //   GetTargetResistance (d1 = resist)
    //     GetResistance
    
    
    // StatusResistPercents:
    //         dc.b 100 = 37%
    //         dc.b 50  = 18%
    //         dc.b 0   = 0%
    //         dc.b 125 = 46%
    
    // (SRP * 37) / 100
    
    // ---------------------------
    
    // ElemResistPercents:
    //         dc.b 100
    //         dc.b 75
    //         dc.b 50
    //         dc.b 125
}
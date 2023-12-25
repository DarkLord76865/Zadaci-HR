use crate::shared::structures::Year;

pub mod day_01;
pub mod day_02;
pub mod day_04;

use day_01::day_01;
use day_02::day_02;
use day_04::day_04;


pub fn year_2019() -> Year {
    Year::new(
        2019,
        vec![
            day_01(),
            day_02(),
            day_04(),
        ],
    )
}
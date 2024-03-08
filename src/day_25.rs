use crate::general_helpers::read_day_input_lines;

const DAY: u8 = 25;
// problem at https://adventofcode.com/2022/day/25
// SNAFU numbers!

fn snafu_to_decimal(n: &str)->i64{
    let mut power = n.len() as u32 - 1;
    let mut out = 0;
    for c in n.chars(){
        match c{
            '=' => out -= 5_i64.pow(power) * 2,
            '-' => out -= 5_i64.pow(power),
            '\n' => {},
            num => out += 5_i64.pow(power) * num.to_string().parse::<i64>().unwrap(),
        }
        if power == 0{ break }
        else { power -= 1}
    }
    out
}
fn base_5_to_str(n: i64)-> char{
    match n{
        0 => '2',
        1 => '1',
        2 => '0',
        3 => '-',
        4 => '=',
        other => panic!("the number {other} should not be possible in base 5")
    }
}


fn decimal_to_snafu(n: i64)-> String {
    // I have to think about this more before tyring it.
    // What a terrible number system.
    let n_decimals = ((2 * n) as f64).log(5.0).ceil() as u32;
    let mut top = 0;
    for p in 0..n_decimals{ top += 2 * 5_i64.pow(p) }
    let mut diff = top - n;
    // println!("top: {top}");
    // println!("diff: {diff}");
    let mut base_5 = Vec::new();
    for p in (0..n_decimals).rev(){
        base_5.push(diff / 5_i64.pow(p));
        diff %= 5_i64.pow(p);
    }
    //println!("base_5: {base_5:?}");
    base_5.iter().map(|n| base_5_to_str(*n)).collect::<String>()
}

pub(crate) fn part_1() {
    let mut dec_sum = 0;
    if let Ok(lines) = read_day_input_lines(DAY) {
        for line in lines.flatten() {
            dec_sum += snafu_to_decimal(line.trim());
        }
    }

    println!("Day {DAY} Part 1: {}", decimal_to_snafu(dec_sum));
}

pub(crate) fn part_2() {
    println!("Day {DAY}: There is no Part 2 on Christmas!")
}

#[cfg(test)]
mod tests {
    use crate::day_25::{decimal_to_snafu, snafu_to_decimal};

    #[test]
    fn test_snafu_to_decimal(){
        assert_eq!(1,snafu_to_decimal("1"));
        assert_eq!(2,snafu_to_decimal("2"));
        assert_eq!(3,snafu_to_decimal("1="));
        assert_eq!(314159265,snafu_to_decimal("1121-1110-1=0"));
    }

    #[test]
    fn test_decimal_to_snafu_to_decimal(){
        let test_nums = [1,2,5,89,12463,467758,2379346,134537,845,9,5];
        for n in test_nums.iter(){
            assert_eq!(*n,snafu_to_decimal(&decimal_to_snafu(*n)));
        }
    }

}
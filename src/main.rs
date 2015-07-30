extern crate docopt;
extern crate rand;

use std::io::Write;
use std::io::stdout;

use std::env::args;
use std::process::exit;

use rand::Rng;

/*
  We could have used BitVec but it is unstable at the moment of development.
*/

static VERSION: &'static str =
    "zobrist-gen 0.1 by Eduardo ES Riccardi (https://github.com/kuking/zobrist-gen)";

static USAGE: &'static str = "
Usage: zobrist-gen [options]
       zobrist-gen (-h | --help)

Options: -b <nbits>  Bits to generate in the output, default 64
         -l <min>    Minimum number of different bits between all generated numbers numbers
                     (Hamming distance) - default 20
         -u <max>    Maximum Hamming distance between values - default 45
         -q <qty>    Quantity of numbers to generate - default 1083

         i.e. zobrist-gen -b 32 -l 10 -u 24 -q 1083
          or: zobrist-gen -b 128 -l 40 -u 90 -q 1083
          or: zobrist-gen -b 16 -l 5 -u 12 -q 243
";

pub fn gen_random(bits :usize) -> Vec<bool> {
    let mut rng = rand::thread_rng();
    return (0..bits).map(|_|rng.gen::<bool>()).collect();
}

pub fn hamming_distance(a :&Vec<bool>, b :&Vec<bool> ) -> usize {
    return a.iter().zip(b.iter()).filter( |&(&ai,&bi)| ai!=bi ).count();
}

pub fn as_base_2(bits : &Vec<bool>) -> String {
    String::from_utf8(
        bits.iter().map(|bit| if *bit { '1' as u8}  else { '0' as u8} ).collect())
    .unwrap_or(String::new())
}

pub fn as_base_16(bits :&Vec<bool>) -> String {
    let mut new_vec = bits.clone();
    while new_vec.len() % 4 != 0 {
        new_vec.insert(0, false);
    }
    String::from_utf8(
        new_vec.chunks(4).map(|h| {
            let digit = h[0] as u8 * 8 + h[1] as u8 * 4 + h[2] as u8 * 2 + h[3] as u8; // ugly use serialise?
            if digit < 10 {
                return '0' as u8 + digit;
            } else {
                return 'a' as u8 + (digit-10);
            }
        }).collect())
    .unwrap_or(String::new())
}

pub fn strbools (st : &str) -> Vec<bool> {
    return st.chars().map( |c| c == '1' ).collect();
}

fn build_zobrist_numbers(bits :usize, bits_min :usize, bits_max :usize, qty :usize) {

    let mut all_numbers : Vec<Vec<bool>> = vec!();

    while all_numbers.len() < qty {

        let number = gen_random(bits);

        let is_ok = all_numbers.iter()
            .map(|x| hamming_distance(&x, &number) )
            .all(|d| bits_min <= d && bits_max >= d );

        if is_ok {
            println!("{:5}] {} [{:2.1}%", all_numbers.len(), as_base_2(&number), all_numbers.len() as f32 * 100.0 / qty as f32);
            all_numbers.push(number);
        }
    }

    language_print (&all_numbers, bits);
    if (bits <= 64) {
        random_sanity_check( &to_vec_u64(&all_numbers) );
    } else {
        println!("");
        println!("Sanity check won't be done for sets of >64 bits.");
        println!("Please choose '-l' & '-r' accordingly.");
    }
}

pub fn language_print(numbers : &Vec<Vec<bool>>, bits :usize) {

//0x3d, 0x99, 0x76, 0x1f, 0xeb, 0xfd, 0xb4, 0xf5, 0x7f, 0x09, 0x7a, 0x98, 0xfa, 0x33, 0x13, 0x6a,
//0xd7fb, 0xbb82, 0x35e5, 0x1734, 0xcd36, 0x1ee3, 0xcd81, 0xf456, 0xbfa6, 0x8b70, 0xeaa8, 0x55c5,
//0x39e2f4cd, 0xe96a5262, 0xd9eb479c, 0xe3023e9e, 0xcc7bb61e, 0x53d117be, 0x5ffb8fd4, 0x9186254c,
//0x348a74dd0019c886, 0x01da919cee743dbf, 0x029aa4fceb219297, 0xedc8f8db68bd15ae,
//0x76111e6e2ebb9771e65bbd8970bc0896, 0x65b85e1007501d05195dcaa236fcf746,
//0xb50a6a0476b6078f8f642582ef327bd0a4c3994339dd7ec633bf5de4c366b127,

    println!("");
    println!(">-+");
    println!("  | Friendly for your favourite compiler:");
    println!("  +--------------------------------------->");
    println!("");
    let max_per_row;
    if      bits > 255 { max_per_row = 1; }
    else if bits > 127 { max_per_row = 2; }
    else if bits > 63  { max_per_row = 4; }
    else if bits > 31  { max_per_row = 8; }
    else if bits > 15  { max_per_row = 12; }
    else { max_per_row = 16; }

    let mut count = 0;
    for number in numbers {
        count = count + 1;
        print!("0x{}, ", as_base_16(&number) );
        if count % max_per_row == 0 {
            println!("");
        }
    }
    println!("");
}

pub fn to_vec_u64 (numbers : &Vec<Vec<bool>>) -> Vec<u64> {
    numbers.iter().map(|n| u64::from_str_radix(&format!("{}", as_base_16(n)), 16).unwrap()).collect()
}

pub fn check_no_zero_xor_for(numbers : &Vec<u64>, comb : &Vec<usize>) {
    let real_numbers : Vec<u64> = comb.iter().map(|&i| *numbers.get(i).unwrap()).collect();
    let mut xor : u64 = 0;
    let mut count = 0;
    for r in &real_numbers {
        xor = xor ^ r;
        count = count + 1;
        if xor == 0 {
            println!("\n");
            println!("given numbers = {:?}", &real_numbers);
            println!("or idx = {:?}", &comb);
            println!("Using the first {} numbers, they will XOR into 0", count);
            println!("Exiting ... definitelly not a good set of numbers; perhaps different '-u' & '-l' values?");
            exit(-1);
        }
    }
}

pub fn random_sanity_check (numbers : &Vec<u64>) {
    println!("");
    println!("Sanity check ... Checking if a random combination of the numbers will XOR into 0");
    println!("(!) This should not be a replacement to a well selected pair of '-l' & '-u' params");
    println!("");

    let mut rng = rand::thread_rng();
    let mut count = 0;
    let mut ch = '-';
    let len = numbers.len();

    println!("I will check 100K times; you might want to ctrl-C (a tick is 1k)");
    while count < 100000 {

        let mut c : Vec<usize> = (1..((rng.gen::<usize>() % len)+5)).map(|_|rng.gen::<usize>() % len).collect();
        c.sort();
        c.dedup();
        if c.len() < 1 { break; }

        count = count + 1;
        if count % 1000 == 0 {
            stdout().write(&[ch as u8]); stdout().flush();
        }
        if count % 25000 == 0 {
            stdout().write(&['\r' as u8]); stdout().flush();
            if ch == '-' { ch = '\\' }
            else if ch == '\\' { ch = '|' }
            else if ch == '|' { ch = '/' }
            else if ch == '/' { ch = '-' }
        }
        check_no_zero_xor_for(numbers, &c);
    }
    println!("\n");
    println!("Sanity passed; Generated numbers might be good enough for a Zobrist hash.")
}



fn parse_params() -> (usize, usize, usize, usize) {

    println!("{}", VERSION);

    let args = docopt::Docopt::new(USAGE)
           .and_then(|d| d.argv(args().into_iter()).parse())
           .unwrap_or_else(|e| e.exit());

   let bits;
   match args.get_str(&"-b").parse::<usize>() {
       Ok(bits_p) => bits = bits_p,
       Err(_)     => bits = 64
   }

   let bits_min;
   match args.get_str(&"-l").parse::<usize>() {
       Ok(bits_p) => bits_min = bits_p,
       Err(_)     => bits_min = 20
   }

   let bits_max;
   match args.get_str(&"-u").parse::<usize>() {
       Ok(bits_p) => bits_max = bits_p,
       Err(_)     => bits_max = 45
   }

   let qty;
   match args.get_str(&"-q").parse::<usize>() {
       Ok(qty_p)  => qty = qty_p,
       Err(_)     => qty = 1083
   }

   if bits_min >= bits {
       println!("I'm sorry but the Hamming distance has to be lower than the max. bits");
       exit(-1);
   }

   if bits_min >= bits_max {
       println!("I'm sorry but lower Hamming distance has to be lower than the maximum provided ({} should be lower than {})", bits_min, bits_max);
       exit(-1);
   }

   println!("Generating {} bits numbers with a Hamming distance between {} and {}.", bits, bits_min, bits_max);

   return (bits,bits_min, bits_max, qty);
}

fn main() {
    let (bits, bits_min, bits_max, qty) = parse_params();
    build_zobrist_numbers(bits, bits_min, bits_max, qty);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn gen_random_test() {
        assert_eq!(10, gen_random(10).len());
        assert!(gen_random(10) != gen_random(10));
    }

    #[test]
    fn strbools_test() {
        assert_eq!(vec![true, false, false, true], strbools(&"1001"));
        assert_eq!(vec![true, true, true, true, true], strbools(&"11111"));
        assert_eq!(vec![false, false, false, false, false], strbools(&"00000"));
        assert!(strbools(&"").len() == 0);
    }

    #[test]
    fn as_base_2_test() {
        assert_eq!("10101111000", as_base_2(&strbools(&"10101111000")));
        assert_eq!("01010101010101", as_base_2(&strbools(&"01010101010101")));
    }

    #[test]
    fn as_base_16_test() {
        // trailing zeros
        assert_eq!("1",  as_base_16(&strbools(&"1")));
        assert_eq!("3",  as_base_16(&strbools(&"11")));
        assert_eq!("7",  as_base_16(&strbools(&"111")));
        assert_eq!("f",  as_base_16(&strbools(&"1111")));
        assert_eq!("1f", as_base_16(&strbools(&"11111")));
        assert_eq!("3f", as_base_16(&strbools(&"111111")));
        // bigger numbers
        assert_eq!("ff", as_base_16(&strbools(&"11111111")));
        assert_eq!("ff00", as_base_16(&strbools(&"1111111100000000")));
        assert_eq!("aa", as_base_16(&strbools(&"10101010")));
        assert_eq!("15576baada", as_base_16(&strbools(&"1010101010111011010111010101011011010")));
        assert_eq!("deadbeef", as_base_16(&strbools(&"11011110101011011011111011101111")));
    }

    #[test]
    fn hamming() {
        assert_eq!(0, hamming_distance ( &strbools(&"1001"), &strbools(&"1001")) );
        assert_eq!(0, hamming_distance ( &strbools(&"10000001"), &strbools(&"10000001")) );
        assert_eq!(8, hamming_distance ( &strbools(&"01111110"), &strbools(&"10000001")) );
        assert_eq!(8, hamming_distance ( &strbools(&"00000000"), &strbools(&"11111111")) );
        assert_eq!(8, hamming_distance ( &strbools(&"10101010"), &strbools(&"01010101")) );
        assert_eq!(4, hamming_distance ( &strbools(&"00001111"), &strbools(&"00000000")) );
    }

}

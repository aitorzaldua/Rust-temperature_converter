const ZERO_BOTTLES: &str = "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n";

const ONE_BOTTLE: &str = "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n";

const TWO_BOTTLES: &str = "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n";

pub fn verse(n: u32) -> String {

    match n {
        0 => return ZERO_BOTTLES.to_string(),
        1 => return ONE_BOTTLE.to_string(),
        2 => return TWO_BOTTLES.to_string(),
        _ => return format!(
            "{} bottles of beer on the wall, \
            {} bottles of beer.\nTake one down and pass it around, \
            {} bottles of beer on the wall.\n",
            n, n, n-1),
    }
  
}

pub fn sing(start: u32, end: u32) -> String {
    
    let mut song: String = "".to_owned();
    
    for n in (end..start+1).rev() {
        song.push_str(&verse(n));
        song.push_str("\n");
    }

    song.pop();

    return song;

}
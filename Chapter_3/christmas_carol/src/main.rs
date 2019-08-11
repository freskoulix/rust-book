static SONG:[&str; 12] = [
"On the PLACEHOLDER day of Christmas my true love sent to me
A partridge in a pear tree",
"On the PLACEHOLDER day of Christmas my true love sent to me
Two turtle doves
And a partridge in a pear tree",
"On the PLACEHOLDER day of Christmas my true love sent to me
Three French hens, two turtle doves
And a partridge in a pear tree",
"On the PLACEHOLDER day of Christmas my true love sent to me
Four calling birds, three French hens, two turtle doves
And a partridge in a pear tree",
"On the PLACEHOLDER day of Christmas my true love sent to me
Five gold rings, four calling birds, three French hens, two turtle doves
And a partridge in a pear tree",
"On the PLACEHOLDER day of Christmas my true love sent to me
Six geese a laying, five gold rings, four calling birds
Three French hens, two turtle doves
And a partridge in a pear tree",
"On the PLACEHOLDER day of Christmas my true love sent to me
Seven swans a swimming, six geese a laying, five gold rings
Four calling birds, three French hens, two turtle doves
And a partridge in a pear tree",
"On the PLACEHOLDER day of Christmas my true love sent to me
Eight maids a milking, seven swans a swimming, six geese a laying
Five gold rings, four calling birds, three French hens, two turtle doves
And a partridge in a pear tree",
"On the PLACEHOLDER day of Christmas my true love sent to me
Nine drummers drumming",
"On the PLACEHOLDER day of Christmas my true love sent to me
Ten pipers piping
Nine drummers drumming, ten pipers piping
Drumming, piping, drumming, piping
Eight maids a milking, seven swans a swimming, six geese a laying
Five gold rings, four calling birds, three French hens, two turtle doves
And a partridge in a pear tree",
"On the PLACEHOLDER day of Christmas my true love sent to me
Eleven ladies dancing, ten pipers piping, nine drummers drumming
Eight maids a milking, seven swans a swimming, six geese a laying
Five gold rings, four calling birds, three French hens, two turtle doves
And a partridge in a pear tree",
"On the PLACEHOLDER day of Christmas my true love sent to me
Twelve Lords a leaping, eleven ladies dancing, ten pipers piping
Nine, drummers drumming, eight maids a milking
Seven swans a swimming, six geese a laying
And five gold rings, four calling birds, three French hens, two turtle doves
And a partridge in a pear tree, and a partridge in a pear tree"
];

fn get_day(day_index: i32) -> &'static str {
    match day_index {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "forth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => "X"
    }
}

fn main() {
    println!("============================");
    println!("The Twelve Days of Christmas");
    println!("============================");
    println!();

    for i in 0..12 {
        let day_index = i + 1;
        let day = get_day(day_index);
        let turn = SONG[i as usize];
        let turn = turn.replace("PLACEHOLDER", day);
        println!("{}\n", turn);
    }
}

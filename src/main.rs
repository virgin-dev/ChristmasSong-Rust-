fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth",
        "sixth", "seventh", "eighth", "ninth", "tenth",
        "eleventh", "twelfth"
    ];

    let gifts = [
        "a Partridge in a Pear Tree",
        "Two Turtle Doves",
        "Three French Hens",
        "Four Calling Birds",
        "Five Gold Rings",
        "Six Geese a Laying",
        "Seven Swans a Swimming",
        "Eight Maids a Milking",
        "Nine Ladies Dancing",
        "Ten Lords a Leaping",
        "Eleven Pipers Piping",
        "Twelve Drummers Drumming"
    ];

    for i in 0..gifts.len() {
        println!("On the {} day of Christmas", days[i]);
        println!("My true love gave to me:");

        for j in (0..=i).rev() {
            if j == 0 && i != 0 {
                println!("And {}", gifts[j]);
            } else {
                println!("{}", gifts[j]);
            }
        }

        println!("");
    }
}
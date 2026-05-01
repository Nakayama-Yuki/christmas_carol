// ループを使用して、クリスマスの12日間の歌詞を出力するプログラム

fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let gifts = [
        "a Partridge in a Pear Tree",
        "two Turtle Doves",
        "three French Hens",
        "four Calling Birds",
        "five Golden Rings",
        "six Geese a-Laying",
        "seven Swans a-Swimming",
        "eight Maids a-Milking",
        "nine Ladies Dancing",
        "ten Lords a-Leaping",
        "eleven Pipers Piping",
        "twelve Drummers Drumming",
    ];

    for i in 0..12 {
        println!(
            "On the {} day of Christmas, my true love sent to me:",
            days[i]
        );
        for j in (0..=i).rev() {
            if j == 0 && i > 0 {
                print!("and ");
            }
            println!("{}", gifts[j]);
        }
        // 各日ごとに空行を追加して、歌詞の区切りを明確にする
        // 中身を何も書かずにprintln!()を呼び出すと、空行が出力されるので改行される
        println!();
    }
}

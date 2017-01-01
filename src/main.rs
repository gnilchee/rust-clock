extern crate time;

use std::cmp::Ordering;

fn main() {
    loop {
        let now = time::now();

        let h   = now.tm_hour;
        let m   = now.tm_min;

        let hour = match h {
            0   => "twelve",
            1   => "one",
            2   => "two",
            3   => "three",
            4   => "four",
            5   => "five",
            6   => "six",
            7   => "seven",
            8   => "eight",
            9   => "nine",
            10  => "ten",
            11  => "eleven",
            12  => "twelve",
            13  => "one",
            14  => "two",
            15  => "three",
            16  => "four",
            17  => "five",
            18  => "six",
            19  => "seven",
            20  => "eight",
            21  => "nine",
            22  => "ten",
            23  => "eleven",
            _   => "",
        };

        let minute = match m {
            0   => "o' clock",
            1   => "o' one",
            2   => "o' two",
            3   => "o' three",
            4   => "o' four",
            5   => "o' five",
            6   => "o' six",
            7   => "o' seven",
            8   => "o' eight",
            9   => "o' nine",
            10  => "ten",
            11  => "eleven",
            12  => "twelve",
            13  => "thirteen",
            14  => "fourteen",
            15  => "fifteen",
            16  => "sixteen",
            17  => "seventeen",
            18  => "eighteen",
            19  => "nineteen",
            20  => "twenty",
            21  => "twenty-one",
            22  => "twenty-two",
            23  => "twenty-three",
            24  => "twenty-four",
            25  => "twenty-five",
            26  => "twenty-six",
            27  => "twenty-seven",
            28  => "twenty-eight",
            29  => "twenty-nine",
            30  => "thirty",
            31  => "thirty-one",
            32  => "thirty-two",
            33  => "thirty-three",
            34  => "thirty-four",
            35  => "thirty-five",
            36  => "thirty-six",
            37  => "thirty-seven",
            38  => "thirty-eight",
            39  => "thirty-nine",
            40  => "forty",
            41  => "forty-one",
            42  => "forty-two",
            43  => "forty-three",
            44  => "forty-four",
            45  => "forty-five",
            46  => "forty-six",
            47  => "forty-seven",
            48  => "forty-eight",
            49  => "forty-nine",
            50  => "fifty",
            51  => "fifty-one",
            52  => "fifty-two",
            53  => "fifty-three",
            54  => "fifty-four",
            55  => "fifty-five",
            56  => "fifty-six",
            57  => "fifty-seven",
            58  => "fifty-eight",
            59  => "fifty-nine",
            _   => "",
        };

        let am_pm   = match h.cmp(&12) {
            Ordering::Less      => "in the morning",
            Ordering::Greater   => "in the evening",
            Ordering::Equal     => "in the evening",
        };

        println!("{} {} {}", hour, minute, am_pm);

        let one_second = std::time::Duration::new(1, 0);
        std::thread::sleep(one_second);
    }
}

#[allow(unused)]
fn main() {
    {
        let mut x = 5;

        println!("The value of x is: {}", x);

        x = 6;

        println!("The value of x is: {}", x);

        const MAX_POINTS: u32 = 100_000;

        let x = x + 1;
        let x = x * 2;

        let spaces = "   ";
        let spaces_ = spaces.len();
    }
    {
        let number: u32 = "42".parse().unwrap();
        let x: i8 = 42;
        let y: u128 = 999999999999999999999999999999999999;

        let z = 1usize;

        let y = 0x42ff;
        let x = 0b1111_0000;
        let x = b'A';

        let flag = false;
        let a = 2.0;
        let b: f32 = 3.0;

        let res = 1.0 / (2u8 as f32);

        let c = 'z';
        let z = 'ℤ';
        let heart_eyed_cat = '😻';

        let tup = (500, 6.4, 1);
        let (x, y, z) = tup;

        let five_hundred = tup.0;

        let arr = [1, 2, 3, 4, 5];

        let first = arr[0];

        let index = 10;
        let element = arr[index];
    }
    {
        another_function(5, 6);

        let x = five();
    }
    {
        // Kommentar
        /* Kommentar */
    }
    {
        let number = 3;
        if number < 5 {
            println!("smaller 5");
        } else {
            println!("greater 5");
        }

        let condition = true;
        let number = if condition { 5 } else { 6 };

        let mut counter = 3;
        while counter != 0 {
            println!("{}!", counter);
            counter -= 1;
        }

        counter = 0;
        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        let a = [10, 20, 30, 40, 50];
        for element in a.iter() {
            println!("the value is: {}", element);
        }

        for i in 0..10 {
            println!("{}", i)
        }
    }
}

fn another_function(x: i32, y: i32) {
    println!("The value of x and y is: {} and {}", x, y);
}

fn five() -> i32 {
    5
}

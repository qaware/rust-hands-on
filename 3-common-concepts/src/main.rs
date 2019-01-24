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
}



pub fn decimal_to_8bit_binary(num: &u8) -> [u8; 8] {
    let mut x = *num;
    let mut arr: [u8; 8] = [0; 8];
    let mut count = 7;

    while x > 0 {
        arr[count] = x % 2;
        x = &x / 2;
        count -= 1;
    }

    arr
}

pub fn decimal_to_6bit_binary(num: &u8) -> [u8; 6] {
    let mut x = *num;
    let mut arr: [u8; 6] = [0; 6];
    let mut count = 6;

    while x > 0 {
        count -= 1;
        arr[count] = x % 2;
        x = &x / 2;
    }

    arr
}

pub fn to_decimal_6bit_binary(arr: [u8; 6]) -> u8 {
    let mut num: u8 = 0;
    let mut count: u32 = 6;

    for bit in &arr {
        count -= 1;
        if *bit == 1 {
            num += 2_u8.pow(count)
        }
    }

    num
}

pub fn to_decimal_8bit_binary(arr: &[u8; 8]) -> u8 {
    let mut num: u8 = 0;
    let mut count: u32 = 8;

    for bit in arr {
        count -= 1;
        if *bit == 1 {
            num += 2_u8.pow(count)
        }
    }

    num
}


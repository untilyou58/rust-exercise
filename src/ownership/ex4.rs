//Exercise 4
// Mục đích : giải quyết vấn đề ownership và borrowing ko dùng clone()
// Logic hiện tại đang sai (cho 1 vec -> đảo chiều vector đó)
pub fn main() {
    let a = &mut vec![1, 2, 3, 4, 5];
    let (a, _i) = test(a);
    println!("{:?}", a);
}

pub fn test(a: &mut Vec<u8>) -> (Vec<u8>, i32) {
    let mut b: Vec<u8> = Vec::new();
    let mut c: u8 = 0;
    loop {
        if a.len() == 0 {
            break;
        }
        let d = a.pop().unwrap();
        c = c + d;
        b.push(d);
    }
    (b, c as i32)
}

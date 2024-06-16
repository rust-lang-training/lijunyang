use rand::{self, Rng};
fn main() {
    let mut arr = [1, 99, 9, 12, 44];

    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
    println!("{:?}", arr);
    let mut arr1 = gen_arr();
    println!("随机生成的数组为：{:?}", arr1);
    for i in 0..arr1.len() {
        for j in 0..arr1.len() - i - 1 {
            if arr1[j + 1] < arr1[j] {
                arr1.swap(j, j + 1);
            }
        }
    }
    println!("排序后的数组为：{:?}", arr1);
}

fn gen_arr() -> [u32; 10] {
    let mut arr: [u32; 10] = [5; 10];
    println!("随机生成10个数值");
    for i in 0..10 {
        arr[i] = rand::thread_rng().gen_range(1..100);
    }
    arr
}

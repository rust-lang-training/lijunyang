fn main() {
    let mut arr1 = [1, 99, -3, 10, 28];
    let mut arr2 = [8; 5];
    let mut arr3: [i32; 0] = [];
    let mut arr4 = [1];

    // 测试 max_val
    assert_eq!(99, max_val(&arr1).unwrap());
    assert_eq!(8, max_val(&arr2).unwrap());
    assert_eq!(None, max_val(&arr3));
    assert_eq!(1, max_val(&arr4).unwrap());

    // 测试 max_val_ref
    assert_eq!(99, *max_val_ref(&arr1).unwrap());
    assert_eq!(8, *max_val_ref(&arr2).unwrap());
    assert_eq!(None, max_val_ref(&arr3));
    assert_eq!(1, *max_val_ref(&arr4).unwrap());

    // 测试 min_val
    assert_eq!(-3, min_val(&arr1).unwrap());
    assert_eq!(8, min_val(&arr2).unwrap());
    assert_eq!(None, min_val(&arr3));
    assert_eq!(1, min_val(&arr4).unwrap());

    // 测试 min_val_ref
    assert_eq!(-3, *min_val_ref(&arr1).unwrap());
    assert_eq!(8, *min_val_ref(&arr2).unwrap());
    assert_eq!(None, min_val_ref(&arr3));
    assert_eq!(1, *min_val_ref(&arr4).unwrap());

    // 测试 avg
    assert_eq!(27.0, avg(&arr1));
    assert_eq!(8.0, avg(&arr2));
    assert_eq!(0.0, avg(&arr3));
    assert_eq!(1.0, avg(&arr4));

    // 测试 map_double
    assert_eq!([2, 198, -6, 20, 56], map_double(&mut arr1));
    assert_eq!([16; 5], map_double(&mut arr2));
    assert_eq!([] as [i32; 0], map_double(&mut arr3));
    assert_eq!([2], map_double(&mut arr4));
}

fn max_val(arr: &[i32]) -> Option<i32> {
    if arr.is_empty() {
        return None;
    }
    let mut max = arr[0];
    for i in 1..arr.len() {
        let cur = arr[i];
        if cur > max {
            max = cur;
        }
    }
    Some(max)
}
fn max_val_ref(arr: &[i32]) -> Option<&i32> {
    if arr.is_empty() {
        return None;
    }
    let mut max_index = 0;
    for i in 1..arr.len() {
        let cur = arr[i];
        if cur > arr[max_index] {
            max_index = i;
        }
    }
    Some(&(arr[max_index]))
}
fn min_val(arr: &[i32]) -> Option<i32> {
    if arr.is_empty() {
        return None;
    }
    let mut min = arr[0];
    for i in 1..arr.len() {
        let cur = arr[i];
        if cur < min {
            min = cur;
        }
    }
    Some(min)
}
fn min_val_ref(arr: &[i32]) -> Option<&i32> {
    if arr.is_empty() {
        return None;
    }
    let mut min_index = 0;
    for i in 1..arr.len() {
        let cur = arr[i];
        if cur < arr[min_index] {
            min_index = i;
        }
    }
    Some(&(arr[min_index]))
}

fn avg(arr: &[i32]) -> f64 {
    if arr.is_empty() {
        return 0.0;
    }
    // let sum: i32 = arr.iter().sum();
    let mut sum = 0;
    for i in 0..arr.len() {
        sum += arr[i];
    }
    sum as f64 / arr.len() as f64
}

fn map_double(arr: &mut [i32]) -> &[i32] {
    if arr.is_empty() {
        return arr;
    }
    for i in 0..arr.len() {
        arr[i] *= 2;
    }
    arr
    // arr.iter_mut().for_each(|x| *x *= 2);
    // Some(arr)
}

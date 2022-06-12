// Find whether an array is subset of another array

pub fn binary_search(arr: &[i32], low: i32, high: i32, key: i32) -> bool {
    if low > high {
        return false;
    }
    let mid = (low + high) / 2;
    if arr[mid as usize] == key {
        return true;
    } else if arr[mid as usize] > key {
        return binary_search(arr, low, mid - 1, key);
    } else {
        return binary_search(arr, mid + 1, high, key);
    }
}

fn partition(arr: &mut [i32], low: i32, high: i32) -> i32 {
    let pivot = arr[high as usize];
    let mut i = low;
    for j in low..high {
        if arr[j as usize] <= pivot {
            arr.swap(i as usize, j as usize);
            i += 1;
        }
    }
    arr.swap(i as usize, high as usize);
    i
}

pub fn sort(arr: &mut [i32], low: i32, high: i32) {
    if low < high {
        let pi = partition(arr, low, high);
        sort(arr, low, pi - 1);
        sort(arr, pi + 1, high);
    }
}

pub fn is_subset(arr1: &[i32], arr2: &[i32]) -> bool {
    let arr1_len = arr1.len();
    let arr2_len = arr2.len();
    if arr1_len > arr2_len {
        return false;
    }
    let mut arr1_copy = arr1.to_vec();
    sort(&mut arr1_copy, 0, (arr1_len - 1).try_into().unwrap());
    let mut arr2_copy = arr2.to_vec();
    sort(&mut arr2_copy, 0, (arr2_len - 1).try_into().unwrap());
    for i in 0..arr1_len {
        if !binary_search(&arr2_copy, 0, (arr2_len - 1).try_into().unwrap(), arr1_copy[i as usize]) {
            return false;
        }
    }
    true
}


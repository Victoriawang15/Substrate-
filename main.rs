
// 基础要求：固定类型的数组排序
fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n-i-1 {
            if arr[j] > arr[j+1] {
                arr.swap(j, j+1);
            }
        }
    }
}

// 提高部分：使用template和PartialOrd实现对任意类型的排序
fn bubble_sort_generic<T: PartialOrd>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n-i-1 {
            if arr[j] > arr[j+1] {
                arr.swap(j, j+1);
            }
        }
    }
}

fn main() {
    let mut arr = [5, 1, 4, 2, 8];
    bubble_sort(&mut arr);
    println!("Sorted array: {:?}", arr);

    let mut arr2 = ["banana", "apple", "pear", "orange"];
    bubble_sort_generic(&mut arr2);
    println!("Sorted array: {:?}", arr2);
}

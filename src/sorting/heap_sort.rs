#[allow(dead_code)]
#[allow(unused_variables)]
pub fn heap_sort(arr: &mut [u8]) {
    let len = arr.len();

    for i in (0..len).rev() {
        // heapify(arr, len, i);
    }
}

// fn heapify(arr: &mut [u8], n: usize, i: usize) {
//     let mut max: usize = i;
//     let left: usize = 2 * i + 1;
//     let right: usize = 2 * i + 2;

//     if left < n && arr[max] < arr[left] {
//         max = left;
//     }

//     if right < n && arr[max] < arr[right] {
//         max = right;
//     }

//     if arr[max] != arr[i] {
//         println!("swap");
//         arr.swap(i, max);
//         heapify(arr, n, max);
//     }
// }

// function​ ​maxHeapify​(arr, n, i) { ​
// let​ max = i;
// ​const​ left = ​2​ * i + ​1​;
// const​ right = ​2​ * i + ​2​;

// ​if​ (left < n && arr[max​][] < arr[​left​]) {
// max = left;
// }

// ​if​ (right < n && arr[​max​] < arr[​right​]) {
// max = right;
// }

// ​if​ (max !== i) {
// [arr[​i​], arr[​max​]] = [arr[​max​], arr[​i​]]
// maxHeapify(arr, n, max)
// }

// }

// function​ ​makeHeap​(arr) {
// ​const​ n = arr.length;
// ​for​ (​let​ i=n- ​1​;i>= ​0​;i--){
//      maxHeapify(arr, n, i);
//   }
// }

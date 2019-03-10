#[allow(dead_code)]
#[allow(unused_variables)]
pub fn quick_sort(arr: &mut [u8]) {
    let len = arr.len() - 1;

    qs(arr, 0, len as isize);
}

fn qs(arr: &mut [u8], low: isize, high: isize) {
    if low < high {
        let pivot: isize = partition(arr, low, high);

        qs(arr, low, pivot - 1);
        qs(arr, pivot + 1, high);
    }
}

fn partition(arr: &mut [u8], low: isize, high: isize) -> isize {
    let mut i: isize = low as isize - 1;

    for j in 0..high - 1 {
        if arr[j as usize] <= arr[high as usize] {
            i += 1;
            arr.swap(i as usize, j as usize);
        }
    }
    println!("High{} Low{}", high, low);

    arr.swap((i + 1) as usize, high as usize);

    i + 1
}

/*
int Partition (int arr[], int low, int high) {
    int pivot = arr[high];
    int i = (low -1);

    for (int j = low; j <= (high - 1); j++) {
        if (arr[j] <= pivot) {
            i++;
            Swap (&arr[i], &arr[j]);
        }
    }

    Swap(&arr[i + 1], &arr[high]);

    return (i + 1);
}

void QuickSort(int arr[], int low, int high)
{
    int pivot;

    if (low < high) {
        pivot = Partition(arr, low, high);
        QuickSort(arr, low, pivot - 1);
        QuickSort(arr, pivot + 1, high);
    }
}

int main(int argc, char* argv[])
{
   /* Call Main sorting algo */
QuickSort(arr, 0, count - 1);

printf("Quick sorted array\n");
PrintArray(arr, 0, count, NOPIVOT);
return 0;
}

*/

pub fn shaker_sort<T: Ord>() {}

// _arr: &mut [u8]

// void shakersort(int a[], int n)
// {
//     int p, i;
//     for (p = 1; p <= n / 2; p++)
//     {
//         for (i = p - 1; i < n - p; i++)
//             if (a[i] > a[i+1]){
//                 swap(&a[i], &a[i + 1]);
// 		}
//         for (i = n - p - 1; i >= p; i--)
//             if (a[i] < a[i-1]){
//                 swap(&a[i], &a[i - 1]);
// 		}
//     }
// }

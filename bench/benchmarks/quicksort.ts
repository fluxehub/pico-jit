// Adapted from https://en.wikipedia.org/wiki/Quicksort

function partition(a: StaticArray<i32>, lo: i32, hi: i32): i32 {
  const pivot = a[lo];
  let i = lo - 1;
  let j = hi + 1;

  while (true) {
    do {
      i++;
    } while (a[i] < pivot);

    do {
      j--;
    } while (a[j] > pivot);

    if (i >= j) {
      return j;
    }

    let t = a[i];
    a[i] = a[j];
    a[j] = t;
  }
}

function quicksort(a: StaticArray<i32>, lo: i32, hi: i32): void {
  if (lo >= 0 && hi >= 0 && lo < hi) {
    const p = partition(a, lo, hi);
    quicksort(a, lo, p);
    quicksort(a, p + 1, hi);
  }
}

export function sort(n: i32): StaticArray<i32> {
  let a = new StaticArray<i32>(n);

  for (let i = 0; i < n; i++) {
    a[i] = n - i;
  }

  quicksort(a, 0, n - 1);

  return a;
}

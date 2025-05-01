### While Loops 

```rs 
    let mut count = 0;
    
    while count < 5 {
        println!("count is: {count}");
        count += 1;
    }
```


### Loop

🔂 2. loop (Infinite Loop)
Repeats forever unless you break out.

```rs 

    let mut i = 0;

    loop {
        println!("i is: {i}");
        i += 1;

        if i == 3 {
            break;
        }
    }


```

## for Loop


🔁 3. for Loop
Used for iterating over iterators, like ranges, arrays, vectors, etc.

```rs 
   for i in 0..5 {
        println!("i is: {i}");
    }
```

0..5 is a range (excludes 5).

Want to include the upper bound? Use 0..=5.

```rs
let arr = [10, 20, 30];

for val in arr.iter() {
    println!("value: {val}");
}
```


### Bonus: Loop Labels (for nested control)


```rs
    'outer: for i in 1..3 {
        for j in 1..3 {
            if i + j == 3 {
                break 'outer; // breaks the outer loop
            }
            println!("i: {i}, j: {j}");
        }
    }
```
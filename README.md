# MashHap

Me messing around with hash tables, using [Crafting
Interpreters](https://craftinginterpreters.com/hash-tables.html) as a launching
point.

## Rust benches

| | SeaHash | FNV-1a |
|-|---------|--------|
|set 10000 'a's| 326.48 µs |905.25 µs|
|set random words|716.56 µs|632.29 µs|
|get random words|417.99 µs|357.47 µs|
|delete random words|320.70 µs|261.48 µs|
|all three|1.1819 ms|957.73 µs|


### FNV-1a
```
Benchmarking insert 10000 'a's: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 9.5s, or reduce sample count to 50.
insert 10000 'a's       time:   [94.165 ms 94.361 ms 94.579 ms]
Found 16 outliers among 100 measurements (16.00%)
  3 (3.00%) high mild
  13 (13.00%) high severe

Benchmarking insert 10000 'a's without resizing: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 9.2s, or reduce sample count to 50.
insert 10000 'a's without resizing
                        time:   [91.800 ms 92.101 ms 92.444 ms]
Found 18 outliers among 100 measurements (18.00%)
  3 (3.00%) high mild
  15 (15.00%) high severe
```

### SeaHash
```
insert 10000 'a's       time:   [29.869 ms 29.980 ms 30.112 ms]
Found 14 outliers among 100 measurements (14.00%)
  5 (5.00%) high mild
  9 (9.00%) high severe

insert 10000 'a's without resizing
                        time:   [27.479 ms 27.569 ms 27.671 ms]
Found 10 outliers among 100 measurements (10.00%)
  1 (1.00%) high mild
  9 (9.00%) high severe
```

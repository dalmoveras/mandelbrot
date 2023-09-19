### Euclidean Algorithm
The Euclidean algorithm calculates the greatest common divisor (GCD) of two natural numbers a and b. The greatest common divisor g is the largest natural number that divides both a and b without leaving a remainder.

### Running:
```
z@x:~/../mathematics/euclidean-algorithm$ cargo test -- --nocapture
   Compiling euclidean-algorithm v0.1.0 (/home/../mathematics/euclidean-algorithm)
    Finished test [unoptimized + debuginfo] target(s) in 0.30s
     Running unittests src/lib.rs (target/debug/deps/euclidean_algorithm-a7a059e82d002b45)

running 1 test

Euclidean algorithm: r=74, a=212, b=138
Euclidean algorithm: r=64, a=138, b=74
Euclidean algorithm: r=10, a=74, b=64
Euclidean algorithm: r=4, a=64, b=10
Euclidean algorithm: r=2, a=10, b=4
Euclidean algorithm: r=0, a=4, b=2
 -----------------
Euclidean algorithm: r=90, a=710, b=310
Euclidean algorithm: r=40, a=310, b=90
Euclidean algorithm: r=10, a=90, b=40
Euclidean algorithm: r=0, a=40, b=10
 -----------------
Euclidean algorithm: r=211943424, a=1160718174, b=316258250
Euclidean algorithm: r=104314826, a=316258250, b=211943424
Euclidean algorithm: r=3313772, a=211943424, b=104314826
Euclidean algorithm: r=1587894, a=104314826, b=3313772
Euclidean algorithm: r=137984, a=3313772, b=1587894
Euclidean algorithm: r=70070, a=1587894, b=137984
Euclidean algorithm: r=67914, a=137984, b=70070
Euclidean algorithm: r=2156, a=70070, b=67914
Euclidean algorithm: r=1078, a=67914, b=2156
Euclidean algorithm: r=0, a=2156, b=1078
 -----------------
test tests::it_works ... ok
```

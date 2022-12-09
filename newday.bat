@echo off

cargo init %1 --bin
cd %1 
cd src
copy NUL input.txt
copy NUL test.txt

(
    echo fn day_fn^(input: ^&str^) -^> u32 {
    echo     0
    echo }
    echo:
    echo fn main^(^) {
    echo     println!^("input: {}", day_fn^(include_str!^("input.txt"^)^)^);
    echo }
    echo:
    echo #^[cfg^(test^)^]
    echo mod test {
    echo     use super::*;
    echo:
    echo     #[test]
    echo     fn test^(^) {
    echo         assert_eq!^(day_fn(include_str!("test.txt"^)^), 0^);
    echo     }
    echo }
) > main.rs

cd ..
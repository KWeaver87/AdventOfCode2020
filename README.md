# Advent of Code 2020
Trying to learn Rust by working through [Advent of Code 2020](https://adventofcode.com/2020/). First time I've done an AoC.

## Organizing

I started off doing each day and part in its own branch. Typically end up reworking a lot of part1's work while working on part2. But then changed at day5 to doing one branch for each day. I name each file and module after the title for the day's puzzle. Then use unit tests to run the code.

## Input files

Files should be saved in `inputs\` folder, using a `dayX` format. Loading functions will accept `"dayX"` as a format. e.g., `"day4"` will load `inputs\day4`.

## Puzzle Solutions

- [Day 1: Report Repair](src/report_repair.rs)
- [Day 2: Password Philosophy](src/password_philosophy.rs)
- [Day 3: Toboggan Trajectory](src/toboggan_trajectory.rs)
- [Day 4: Passport Processing](src/passport_processing.rs)
- [Day 5: Binary Boarding](src/binary_boarding.rs)
- [Day 6: Custom Customs](src/custom_customs.rs)
- [Day 7: Handy Haversacks](src/handy_haversacks.rs)

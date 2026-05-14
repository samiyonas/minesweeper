## minesweeper game in your terminal

### Try it out
#### clone
```angular2html
git clone https://github.com/samiyonas/minesweeper
cd minesweeper
```

#### run the release build(if you don't have cargo)
## macOS
```angular2html
./target/release/minesweeper
```
## linux
```angular2html
./target/x86_64-unknown-linux-gnu/release/minesweeper
```
## windows
```angular2html
./target/x86_64-pc-windows-gnu/release/minesweeper.exe
```

#### build and run the game yourself
```angular2html
cargo build --release
./target/release/minesweeper
```

#### this game was inspired by [[this](https://leetcode.com/problems/minesweeper/description/)] leetcode problem
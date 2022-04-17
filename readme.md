# slice reader

A somewhat sane reader/cursor for simple binary parsing.
- No `io::Result`, just `Option`.
- No private. It's just a slice + index with some functions.

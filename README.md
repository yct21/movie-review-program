# Native Solana Program -  Movie Review Program

## Build

```bash
cargo build-sbf
```

## Deploy

```bash
solana program deploy target/sbf-solana-solana/release/movie_review_program.so
```

这个会生成一个ProgramId，这里是需要记录下来，在后面前端调用的时候需要用到。

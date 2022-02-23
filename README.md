# solana-voting
An example of creating Solana programs to build a voting system using Anchor. The system consists of 3 programs which are responsible for voting, vote counting and rewards earning so as to demonstrate how cross-program invocations work.

### Version
- [Solana Tool Suite](https://solana.com/): 1.8.12
- [Rust](https://www.rust-lang.org/): 1.58.0
- [Anchor CLI](https://project-serum.github.io/anchor/): 0.20.1
- [TypeScript](https://www.typescriptlang.org/): 4.3.5

### Installation
Install Solana Tool Suite.
```
sh -c "$(curl -sSfL https://release.solana.com/v1.8.12/install)"
```

Update the `PATH` environment variable to include Solana programs by adding the following command to `.profile` and `.zshrc` in your home directory.
```
export PATH="/Users/celiakwan/.local/share/solana/install/active_release/bin:$PATH"
```
Then run this to make the changes applicable in the shell.
```
source ~/.zshrc
```

Install Rust.
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install Anchor CLI.
```
cargo install --git https://github.com/project-serum/anchor --tag v0.20.1 anchor-cli --locked
```

### Configuration
Generate a key pair.
```
solana-keygen new
```

Set the network to localhost.
```
solana config set --url localhost
```

Each key pair will automatically start with 500,000,000 SOL.

### Build
```
anchor build
```

### Deployment
1. Get the program IDs.
```
solana address -k target/deploy/solana_voting-keypair.json
solana address -k target/deploy/vote_count-keypair.json
solana address -k target/deploy/vote_record-keypair.json
```

2. Update `programs/solana-voting/src/lib.rs`, `programs/vote-count/src/lib.rs` and `programs/vote-record/src/lib.rs` with the corresponding program ID generated above. For example:
```
declare_id!("3yFnjbmi9Fhd999rLMM5hiFen2f1u4LLqTedASi66jx9");
```

3. Update `Anchor.toml` with the program IDs generated above.
```
[programs.localnet]
solana_voting = "3yFnjbmi9Fhd999rLMM5hiFen2f1u4LLqTedASi66jx9"
vote_count = "Ga9XxaFMFg2wFfj6fgHreuLaJw5kBPhNjoBqSJCSZyCN"
vote_record = "8kJ6Ny1TiBaLSJ5Tw3sHyWPvH1iP51rUrY4V48N4nrAR"
```

4. Run a local validator.
```
solana-test-validator
```

5. Deploy the program.
```
anchor deploy
```

### Testing
```
anchor test
```
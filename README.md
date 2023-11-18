# Word Mapper

Tokenize and print out all the tokens and their number of occurences in a file or directory.

Default printing will include all current files with their path

### Flags 
- -C --count Which will print the overall count of the words.
- -S --search will accept keyword to search for

# Examples

## Search

```text
wmapper -S clap .

clap: 1 <-- ./Cargo.toml
libclap: 1 <-- ./target/debug/deps/clap_derive-2e3908ac6b5030f9.d
clap: 52 <-- ./target/debug/deps/clap_derive-2e3908ac6b5030f9.d
clap: 11 <-- ./target/debug/deps/clap-4118f9272b341c70.d
libclap: 1 <-- ./target/debug/deps/clap_builder-8b9728f7d4c14027.d
clap: 218 <-- ./target/debug/deps/clap_builder-8b9728f7d4c14027.d
libclap: 1 <-- ./target/debug/deps/clap_lex-962ce013888c1866.d
clap: 10 <-- ./target/debug/deps/clap_lex-962ce013888c1866.d
clap: 8 <-- ./target/debug/deps/clap_lex-7cb57be013ed76d5.d
libclap: 1 <-- ./target/debug/deps/clap-a17b057d92f6ad34.d
clap: 14 <-- ./target/debug/deps/clap-a17b057d92f6ad34.d
clap: 164 <-- ./target/debug/deps/clap_builder-2f0e5def2c89c4ce.d
clap: 11 <-- ./target/debug/deps/clap-f53a4c9646d03ab0.d
clap: 1 <-- ./target/debug/.fingerprint/wmapper-52e58b92fb245005/bin-wmapper.json
clap: 21 <-- ./target/debug/.fingerprint/wmapper-52e58b92fb245005/output-bin-wmapper
```

## Count

```text
wmapper -C .

pointer: 1
32: 1
exploit: 1
hardcodes: 1
Examples: 1
results: 1
folder: 1
forked: 1
Word: 1
exist: 1
bare: 1
limit: 1
interpret: 1
invoked: 1
1700307670: 1
677: 1
Tag: 1
8143805809435215931: 1
perform: 1
exiting: 1
override: 1
Creating: 1
After: 1
overall: 1
number: 1
constrain: 1
happen: 1
accept: 1
7079075: 1
fxsr: 1
clean: 1
```

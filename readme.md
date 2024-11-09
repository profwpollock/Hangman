# A text-mode game of hangman for one player

The game picks a random word from a word list, then the user gets
seven tries to guess letters in the word.
At anytime, the player can try to guess the whole word instead of
a letter.

The word list was downloaded 2024-11-03 from
[GitHub.com/Tom25/Hangman](https://github.com/Tom25/Hangman/blob/master/wordlist.txt)
Note: had to remove the word "quit" from the list!

The game also displays guessed letters.

## Directions

- First, run `cargo update` and also edit this `readme.md` file,
and optionally the cargo.toml file's project metadata.
- When initial do-nothing project is ready (), run the command:
    ```
        git commit -a -m "Initial Commit"
    ```
As you code:
- Add dependencies with "`cargo add ...`".
- Test with "`cargo test`".
- Security scan with "`cargo audit`" (alias with options is `aud`).
- Scan for bad code with "`cargo clippy`".
- Run with "`cargo run`".
- Generate documentation and open in browser with "`cargo d`"
- Reformat code with "`cargo fmt`".
- Generate SBOM with `gensbom.cmd` Windows script, or run
  "`syft scan dir:. -o cyclonedx-json=SBOM.json`"
  or "`syft scan dir:. -o cyclonedx-json | jq . > SBOM.json`" for
  formatted output.  (Install `syft` and `jq` first.)
  (`cargo sbom >SBOM.txt` also works, but output format isn't json.)
- Create a package of your crate with "`cargo package`"

(Don't forget to commit your changes in Git!)

# Resume converter: convert your resume from Markdown

Quickly and easily convert your resume from Markdown to some more easily
parsable format.

## The why: Form vs Function

Have you ever hit writer's block when attempting to update your resume? Do you
waste time obsessing over the presentation rather than focusing on the actual
content? Would you like to simply write your resume once and see it presented
however you wish, instantly? Are you a geek who loves markdown more than most
children?

Then this project's for you! Well, sort of... I make no attempt at actually
formatting your resume (sorry!). There's loads of advice and frankly I couldn't
be bothered to keep up with it all. What this project does is convert a
markdown representation of your resume into an equavalent TOML (or YAML, or
JSON, we have options) that's much easier for you to parse.

## Usage

```bash
cargo run -- --input ./examples/resume.md --output ./examples/resume.toml
```

By default `--input` is assumed to be `./resume.md`, `--output` is assumed to
be `./resume.toml`. The output file's extension implies the format.

## Supported formats

- .yaml
- .toml
- .json

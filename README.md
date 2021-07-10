# text_to_pdf
Basically what it sounds like. Sucks up all the text files in a directory, dumps them into a single file, and saves it to a file. Which, glaringly, does not actually convert to a PDF. I'll probably use the [lopdf crate](https://crates.io/crates/lopdf).

TODO:
- [ ] Filter out non-text files
- [ ] Quit out if no text files found
- [ ] Add crate to finish the pdf conversion (currently just call enscript and ps2pdf on my local machine to accomplish this)

 Here's the help text:
```
text_to_pdf 0.1.0

USAGE:
    text_to_pdf [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --filter <file-filter>    The pattern for files to concatenate [default: txt]
    -i, --input <input-path>      The path to the files to read [default: ./]
    -o, --output <output-file>    The name of output file [default: output.txt]
```
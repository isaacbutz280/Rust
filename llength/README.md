## llength

A command line tool that was developed because I wanted a way to check line length in my source files, and as an excuse to 
use the [`colored`](https://crates.io/crates/colored) crate.
The purpose of this program is the iterate over the lines of text in a file, and determine if they are "valid". 
A valid line is one that is less than the [length](#length) argument, and will be displayed in green text, while a line longer than the specified
length will be demmed invalid, and printed out in red text. The two arguments taken are a [filename](#filename) and a [length](#length) (optional). Additionally, 
an environment variable named [TAB_SIZE](#tab-size) can be set to modify how long tab stops are. This program has been tested and works on both Ubuntu terminals and Windows PowerShell, however due to the
`colored` crate the output is a little strange on Git bash.

To use the tool, clone into this repository, make your way into the `llength` directory, and use the [cargo](https://doc.rust-lang.org/cargo/) command
```
cargo build --release
```

Then the executable will be found in `llength/target/release` as `llength` or `llength.exe`. See below for more detail. I hope you find llength useful!

---

### Output

The program output will start with a header that looks like this:
```
?|Line Num |Count|Text
```

With...
- ? - If the line is invalid will contain an 'X', or if valid just a space
- Line Num - The line number in the file
- Count - The total length of the line
- Text - Displays the text that the line contained

As stated before, valid lines will be output in green, while invalid lines are output in red. Here is an example output:
![image](https://user-images.githubusercontent.com/80608235/153116765-4b75e5b0-5051-4bab-bb73-f34600175239.png)

#### Filename

The first argument specified is the file to perform the operation on. This must be a valid path, and can not be a directory.

#### Length

This optional argument is the length each line is compared on. If not provided, the default value is 80. A line is deemed valid if it's length is less than that
of the length argument.

#### Tab size

The environment variable `TAB_SIZE` tells the program how long tab stops are spaced apart. On some text editors, hitting tab will take you to the next tab stop, 
spaced every so many spots apart. For [emacs](https://www.gnu.org/software/emacs/manual/html_node/emacs/Tab-Stops.html#:~:text=Emacs%20defines%20certain%20column%20numbers,M%2Di%20(see%20Indentation%20Commands).&text=The%20default%20value%20is%20nil,tab%20stop%20every%208%20columns.)
, tab stops are spaced every 8 characters, so the default length of a tab stop is 8. To change the value, update the environment variable `TAB_SIZE`. Note that 
many modern text editors replace tabs with spaces, so you should not need to worry about this. 

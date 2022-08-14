# Minigrep

This is an implementation of the minigrep project found in
[chapter 12 of the rust book](https://doc.rust-lang.org/book/ch12-00-an-io-project.html).
It will search for a substring in lines of a file.

## Running this Project

The program can be run with the following command.
`<search_str>` is the substring to search for in `<search_file>`.
The final command line argument is optional.
It sets whether the search is sensitive or insensitive.
The default is to use sensitive search.

```bash
cargo run <search_str> <search_file> <sensitive|insensitive>
```
A sample file `poem.txt` is provided.
To search for `to` in `poem.txt` run the following command

```bash
cargo run to poem.txt
```

which will return

```
Are you nobody, too?
How dreary to be somebody!
```

The final argument can be used to make the search insensitive

```bash
cargo run to poem.txt
```

which will return

```
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

This program can use an environment variable `CASE_INSENSITIVE` to set searchs to insensitive.
Passing `sensitive` or `insensitive` as a command line argument will override the environement variable.
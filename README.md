# `fortunelike` v0.2.0 #

It's like the
classic [`fortune`](https://en.wikipedia.org/wiki/Fortune_%28Unix%29)
program, but it's written in Rust (as a learning exercise).  It also
uses a Yaml-format fortune database.

## Usage ##

`fortunelike` outputs a random entry from the fortune database.  The
database path can be chosen with the `-f` / `--dbfile` option or by
setting the `FORTUNELIKE_DB` environment variable.

    $ fortunelike -f example_db.yaml
    You find yourself reading a confusing message.

If no database path is provided, `fortunelike` first defaults to
`$HOME/.config/fortunelike-db` and then `/etc/fortunelike-db`.  If no
database at all is found, or if the first one that is found does not
exist or cannot be parsed, a `[?]` will be output.

`fortunelike` usually ends its output with a newline; if you don't
want this, pass the `-i` / `--inline` option.  This is particularly
useful if you want to include the fortune in the middle of some other
text.

    $ echo "Insert some $(fortunelike -i -f words_db.yaml) text."
    Insert some banana text.

## Fortune databases ##

The fortune database file is a list of strings in Yaml format.  See
`example_db.yaml` and `words_db.yaml` for examples.

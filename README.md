# 󠀼Nedo.html
󠀼Nedo.html is a HTML runtime built as a PoC for people who think markup languages can't be programming languages.

## Hello, World!

```html
<html>
  <body>
    <output>Hello, World!</output>
  </body>
</html>
```

```sh
$ nedo hello-world.html
> Hello, World!
```

The `<output>` tag can be used to print out messages to the stdout.

## Usage

Similarly the `<input>` tag can be used to read content from the stdin.

```html
<html>
  <body>
    <input />
  </body>
</html>
```

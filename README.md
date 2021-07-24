## CSS to code generator

_For the old version of this application, please go [here](https://github.com/scoville/tailwind-generator/tree/master)._

### A type safe CSS to "\*" code generator, tailored for Tailwind.

Generates code from _any_ valid css file (this CLI has been tested against complex CSS files generated by Tailwind). Currently supports TypeScript, ReScript, Elm, PureScript, and Rust (via a `css!` macro that doesn't require code generation, see below).

### Installation notice

This new version can be installed using npm/yarn using this command:

```bash
npm install https://github.com/scoville/tailwind-generator\#v2
```

or

```bash
yarn add https://github.com/scoville/tailwind-generator\#v2
```

### Commands:

To get help:

```bash
style-generator --help
```

```
style-generator

USAGE:
    style-generator [OPTIONS] --input <input> --output-filename <output-filename> --lang <lang>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <input>
            CSS file path or URL to parse and generate code from

    -l, --lang <lang>
            Language used in generated code (elm|purescript|rescript|typescript|typescript-
            type-1|typescript-type-2)"

    -o, --output-directory <output-directory>    Directory for generated code [default: ./]
    -f, --output-filename <output-filename>
            Filename (without extension) used for the generated code
```

`style-generator` uses [env_logger](https://docs.rs/env_logger/0.8.4/env_logger/) under the hood, so you can prefix your command with `RUST_LOG=info` for a more verbose output, the binary is silent by default.

Warning: in PureScript and Elm, the provided filename and directory path will be used as the module name, make sure they follow the name conventions and are capitalized. For example:

```bash
style-generator -i ./styles.css -l purescript -o ./Foo/Bar -f Baz
```

Will generate a `./Foo/Bar/Baz.purs` file that defines a module called `Foo.Bar.Baz`.

### Examples

Display the help message:

```bash
style-generator -h
```

Generates a TypeScript file called `css.ts` in the `generated` folder from the Tailwind css file:

```bash
style-generator -i https://unpkg.com/tailwindcss@^2/dist/tailwind.min.css -l typescript -f css -o generated
```

Same as above but generated from a file:

```bash
style-generator -i ./styles.css -l typescript -f css -o generated
```

Generates a PureScript file and shows logs:

```bash
RUST_LOG=info style-generator -i ./styles.css -l purescript -f Css
```

### Generators

#### TypeScript

`style-generator` offers three flavors for TypeScript code generation, let's see and compare the three solutions.

#### TypeScript (typescript)

A simple generator for TypeScript, it exports an [opaque type](https://en.wikipedia.org/wiki/Opaque_data_type) `CssClass`, a `join` function, and a set of `CssClass` "objects":

```ts
import { join, textBlue100, rounded, border, borderBlue300 } from "./css.ts";

// ...

<div className={join([textBlue100, rounded, border, borderBlue300])}>
  Hello
</div>;
```

Pros:

- Easy to use
- Very flexible
- Compatible with most TypeScript versions
- Safe, you can't pass any string to the `join` function
- Autocompletion

Cons:

- Cost at runtime: `CssClass` are JavaScript objects that help ensuring type opacity
- Cost at runtime: the array has to be joined into a string
- Imports can be verbose (unless you use `import * as ...`)
- Not the "standard" class names, `h-full` becomes `hFull`, etc...

#### TypeScript type 1 (typescript-type-1) (recommended)

This generator doesn't generate any runtime code apart from the `join` function.

```ts
import { join } from "./css.ts";

// ...

<div className={join("text-blue-100", "rounded", "border", "border-blue-300")}>
  Hello
</div>;
```

Pros:

- Easy to use
- Very flexible
- Compatible with most TypeScript versions
- Safe, you can't pass any string to the `tailwind` function
- "Standard" class names
- Light import (you only need the `join` function)
- Autocomplete

Cons:

- Cost at runtime: the classes must be "joined" into a string

#### TypeScript type 2 (typescript-type-2)

This generator doesn't generate any runtime code apart from the `css` function.

```ts
import { css } from "./css.ts";

// ...

<div className={css("text-blue-100 rounded border border-blue-300")}>
  Hello
</div>;
```

Pros:

- Super easy to use
- Safe, you can't pass any string to the `tailwind` function
- "Standard" class names
- Light import (you only need the `css` function)
- No runtime cost at all
- Partial support for autocompletion

Cons:

- Not as flexible as the 2 other generators
- Compatible with TypeScript > 4.1 only
- Type error can be hard to debug
- Doesn't accept multiple spaces (not necessarily a cons for some)

#### PureScript (purescript)

In PureScript, a `CssClass` newtype is exported _without its constructor_ which derives some very useful type classes like Semigroup or Monoid offering a lot of flexibility:

- Simple list of css classes:

```purescript
[ rounded, borderRed100 ]
```

- Add a class conditionally:

```purescript
[ if true then textBlue500 else textRed500 ] -- "text-blue-500"
```

- Add a class only if a condition is met, do nothing otherwise:

```purescript
[ guard true textBlue500 ] -- "text-blue-500"
[ guard false rounded ] -- ""
```

- Handle Maybe, and other Foldable values:

```purescript
[ rounded, fold Nothing ] -- "rounded"
[ rounded, fold $ Right wFull ] -- "rounded w-full"

let mClass = Just borderRed100 in
[ rounded, fold mClass ] -- "rounded border-red-100"
```

Example:

```purescript
import Css (rounded, borderRed100, join)

css :: String
css = join [ rounded, borderRed100 ]
```

#### ReScript (rescript)

_You can also take a look at [this ppx](https://github.com/dylanirlbeck/tailwind-ppx) if you want to skip the code generation step. Both approach (code generation and ppx) have pros and cons._

In ReScript 2 files are generated, one that contains the code and an interface file.

Additionally to the class variables, 2 functions are exposed:

- `join`: takes a list of `cssClass` and returns a string
- `joinOpt`: takes a list of `option<cssClass>` and returns a string

```rescript
open Css

<div className={join([textBlue100, rounded, border, borderBlue300])}>
  {"Hello!"->React.string}
</div>
```

#### ReScript type (rescript-type)

Since ReScript 9.1 we can safely coerce [polymorphic variants to strings](https://rescript-lang.org/blog/release-9-1#polymorphic-variants-for-numbers-and-strings). This generator leverages this new feature.

It's lighter than the other ReScript generator, and it's possible to get class names autocompletion using the [Tailwind IntelliSense](https://marketplace.visualstudio.com/items?itemName=bradlc.vscode-tailwindcss) plugin.

Example:

```rescript
<div className={Css.join([#"text-blue-100", #rounded, #border, #"border-blue-300"])}>
  {"Hello!"->React.string}
</div>
```

#### Elm (elm)

Additionally to the generated classes, you'll get 2 useful functions:

- `classes`: takes a list of css classes and returns an `Html.Attribute msg` that can be used with any html element
- `join`: performs a simple `List CssClass -> String` conversion when you need to compute a class name outside of an html element

```elm
import Css exposing (classes, textBlue100, rounded, border, borderBlue300);

view _model =
  div [ classes [ textBlue100, rounded, border, borderBlue300 ] ]
    [ text "Hello!" ]
```

### No generators

Some languages allow for more flexibility using macros or an other mechanism. Rust, Crystal, or the OCaml languages (Ocaml, ReasonML, and ReScript) are some of those languages, and the `style-generator` library offers support for a few of them.

_ReScript users: this tool doesn't offer any other support than the generator (see above) yet, in the meantime you can take a look at [this ppx](https://github.com/dylanirlbeck/tailwind-ppx)._

#### Rust (rust)

_TODO: Add notice about installation after release._

In Rust, a `style-generator.toml` file is required and must be located at the root of your crate. It's pretty simple (as of today) and should look like this:

```toml
[general]
input = "./styles.css" # or input = {path = "./styles.css"}
```

Notice that urls are also supported, which can come in handy when testing or developing your application as in that case no file is needed:

```toml
[general]
input = {url = "https://unpkg.com/tailwindcss@^2/dist/tailwind.min.css"}
```

If your config file is valid and the css can be found, you can now use the `css!` macro:

```rust
use style_generator_macro::css;

// ...

let style = css!(" rounded  border px-2  py-1");

// Notice that extra white spaces have been removed at compile time
assert_eq!(style, "rounded border px-2 py-1");
```

The css class names are validated and cleaned at compile time, duplicates are removed (and a compiler warning is emitted if you're using Rust nightly) and the whole macro call is replaced by the provided string itself.

_[Yew](https://yew.rs/) users: the `css!` macro can be used instead of the `classes!` one._

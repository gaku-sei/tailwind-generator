## CSS code generator

### A type safe CSS to \* code generator, tailored for Tailwind.

Generates code from _any_ valid css file (this CLI has been tested against complex CSS files generated by Tailwind). Currently supports TypeScript, ReScript, Elm, and PureScript! (Support for Rust on the horizon).

### Commands:

To get help:

`$ tailwind-generator --help`

```
tailwind-generator

USAGE:
    tailwind-generator [OPTIONS] --input <input> --lang <lang>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <input>                        CSS file to parse and generate code from
    -l, --lang <lang>
            Language used in generated code (elm|purescript|rescript|typescript)"

    -o, --output <output>                      Directory for generated code [default: ./]
    -f, --output-filename <output-filename>
            Filename (without extension) used for the generated code [default: Tailwind]
```

`tailwind-generator` uses [env_logger](https://docs.rs/env_logger/0.8.4/env_logger/) under the hood, so you can prefix your command with `RUST_LOG=info` for a more verbose output, the binary is silent by default.

### Generators

#### TypeScript (typescript)

A simple generator for TypeScript, it exports an [opaque type](https://en.wikipedia.org/wiki/Opaque_data_type) `Tailwind`, `tailwind` function build, and a set of `Tailwind` objects:

```ts
import {
  tailwind,
  textBlue100,
  rounded,
  border,
  borderBlue300,
} from "./tailwind.ts";

// ...

<div className={tailwind([textBlue100, rounded, border, borderBlue300])}>
  Hello
</div>;
```

Pros:

- Easy to use
- Very flexible
- Compatible with most TypeScript versions
- Safe, you can't pass any string to the `tailwind` function
- Autocompletation

Cons:

- Cost at runtime: each Tailwind object is a JavaScript object to ensure type opacity
- Cost at runtime: the array has to be joined into a string
- Imports can be verbose (unless you use `import * as ...`)
- Not the "standard" class names, `h-full` becomes `hFull`, etc...

# Dither

Inspired by: <http://www.tannerhelland.com/4660/dithering-eleven-algorithms-source-code/>

## License: MIT

## CLI help:

```bash
dither --help
```

## usage

```bash
dither INPUT [OUTPUT] [-c] [-v] [--ditherer] [--bit_depth]
```

example:

```bash
dither example.png color_atkinson_01.png -c -v
```

## args

### `INPUT`

the path to a PNG

### `OUTPUT`

the path to write the output. this will create or truncate the file at the existing location, if necessary

### `'-c'`, `--color`

Whether to dither in black and white (default) or color.

### `-d, --dither`

The type of ditherer to use. Available options are

- `"floyd"`, `"steinberg"`, `"floydsteinberg"` _(default, floyd-steinberg dithering)_
- `"atkinson"`,
- `"stucki"`,
- `"burkes"`,
- `"jarvis"`, `"judice"`, `ninke"` _Jarvis-Judice-Ninke dithering_

### `-v, --verbose`

Verbose debug output

### `--bit-depth`

Default 1\. Bit depth should be an unsigned integer between 1 and 7\. The number of bits to compress each channel to. Default is 1 (black and white dithering or eight-color).

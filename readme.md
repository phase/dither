# Dither

Inspired by: <http://www.tannerhelland.com/4660/dithering-eleven-algorithms-source-code/>

## License: MIT

## usage

```bash
dither INPUT OUTPUT [-c] [-v] [--ditherer=floyd]
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

- `"floyd"` (default, floyd-steinberg dithering)
- `"atkinson"`,
- `"stucki"`,
- `"burkes"`,

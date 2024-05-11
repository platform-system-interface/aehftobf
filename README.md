# haeftobf

TL;DR: Decode a *h*ex-*A*SCII-*e*ncoded *f*ile *to* a raw *b*inary *f*ile:
```sh
haeftobf foo.asc
```

The output file will have the same name, just with the extension `.bin`.

Your file may contain any number of separate lines.
No spaces or other characters are allowed within a line.
Every line is required to contain an even number of characters, of which two
each represent one byte, hex-encoded. E.g., `42` is the character `B`.
In other words, the resulting file will be half the input file or less in size.

This is useful in scenarios where you may easily transfer ASCII characters, but
you really want binary data. You can use `haeftobf` to dump memory over a UART,
while you also have code running that logs other information, for example.

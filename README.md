`uu` is a command line program for inspecting Unicode text.

## Installation

You can install `uu` via Homebrew.

```
brew install jake-low/tools/uu
```

Alternately, if you have a Rust toolchain installed you can download the source code for a [release](https://github.com/jake-low/uu/releases) and build it with `cargo build`.

## Example

`uu inspect` will read from STDIN and print a line for each code point it finds, with details about that code point.

```
$ echo 'Hi… 你好! Φₑ = ∯ E⋅da 🙌🏽' | uu
GLYPH  CODE POINT  UTF-8 BYTES  NAME                                      BLOCK                                  CATEGORY
H      U+0048      48           LATIN CAPITAL LETTER H                    Basic Latin                            Uppercase Letter
i      U+0069      69           LATIN SMALL LETTER I                      Basic Latin                            Lowercase Letter
…      U+2026      e2 80 a6     HORIZONTAL ELLIPSIS                       General Punctuation                    Other Punctuation
       U+0020      20           SPACE                                     Basic Latin                            Space
你     U+4F60      e4 bd a0     CJK UNIFIED IDEOGRAPH-4F60                CJK Unified Ideographs                 Other Letter
好     U+597D      e5 a5 bd     CJK UNIFIED IDEOGRAPH-597D                CJK Unified Ideographs                 Other Letter
!      U+0021      21           EXCLAMATION MARK                          Basic Latin                            Other Punctuation
       U+0020      20           SPACE                                     Basic Latin                            Space
Φ      U+03A6      ce a6        GREEK CAPITAL LETTER PHI                  Greek and Coptic                       Uppercase Letter
ₑ      U+2091      e2 82 91     LATIN SUBSCRIPT SMALL LETTER E            Superscripts and Subscripts            Modifier Letter
       U+0020      20           SPACE                                     Basic Latin                            Space
=      U+003D      3d           EQUALS SIGN                               Basic Latin                            Math Symbol
       U+0020      20           SPACE                                     Basic Latin                            Space
∯      U+222F      e2 88 af     SURFACE INTEGRAL                          Mathematical Operators                 Math Symbol
       U+00A0      c2 a0        NO-BREAK SPACE                            Latin-1 Supplement                     Space
E      U+0045      45           LATIN CAPITAL LETTER E                    Basic Latin                            Uppercase Letter
⋅      U+22C5      e2 8b 85     DOT OPERATOR                              Mathematical Operators                 Math Symbol
d      U+0064      64           LATIN SMALL LETTER D                      Basic Latin                            Lowercase Letter
a      U+0061      61           LATIN SMALL LETTER A                      Basic Latin                            Lowercase Letter
       U+0020      20           SPACE                                     Basic Latin                            Space
🙌     U+1F64C     f0 9f 99 8c  PERSON RAISING BOTH HANDS IN CELEBRATION  Emoticons                              Other Symbol
🏽     U+1F3FD     f0 9f 8f bd  EMOJI MODIFIER FITZPATRICK TYPE-4         Miscellaneous Symbols and Pictographs  Modifier Symbol
^J     U+000A      0a           <LINE FEED>                               Basic Latin                            Control
```

`uu lookup` takes a UTF-8 glyph or a code point in U+XXXX format, and prints a table of information about it.

```
$ uu lookup U+203D
Glyph:                ‽
Code point:           U+203D
Name:                 INTERROBANG
Block:                General Punctuation
Category:             Other Punctuation (Po)
Bidirectional Class:  OtherNeutral (ON)
Added in version:     1.1.0
UTF-8:                e2 80 bd
UTF-16BE:             20 3d
UTF-16LE:             3d 20
UTF-32BE:             00 00 20 3d
UTF-32LE:             3d 20 00 00
```

## License

The source code for `uu` is available under the ISC license. See the LICENSE file for details.

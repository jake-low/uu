`uu` is a command line program for inspecting Unicode text. It reads UTF-8 from stdin and prints details about each code point it encounters.

## Example

```
$ echo 'Hi… 你好! Φₑ = ∯ E⋅da 🙌🏽' | uu
GLYPH  CODE POINT  UTF-8 BYTES  NAME                                      BLOCK                                  CATEGORY
H      U+0048      48           LATIN CAPITAL LETTER H                    Basic Latin                            Uppercase Letter
i      U+0069      69           LATIN SMALL LETTER I                      Basic Latin                            Lowercase Letter
…      U+2026      e2 80 a6     HORIZONTAL ELLIPSIS                       General Punctuation                    Other Punctuation
       U+0020      20           SPACE                                     Basic Latin                            Space
你     U+4f60      e4 bd a0     CJK UNIFIED IDEOGRAPH-4F60                CJK Unified Ideographs                 Other Letter
好     U+597d      e5 a5 bd     CJK UNIFIED IDEOGRAPH-597D                CJK Unified Ideographs                 Other Letter
!      U+0021      21           EXCLAMATION MARK                          Basic Latin                            Other Punctuation
       U+0020      20           SPACE                                     Basic Latin                            Space
Φ      U+03a6      ce a6        GREEK CAPITAL LETTER PHI                  Greek and Coptic                       Uppercase Letter
ₑ      U+2091      e2 82 91     LATIN SUBSCRIPT SMALL LETTER E            Superscripts and Subscripts            Modifier Letter
       U+0020      20           SPACE                                     Basic Latin                            Space
=      U+003d      3d           EQUALS SIGN                               Basic Latin                            Math Symbol
       U+0020      20           SPACE                                     Basic Latin                            Space
∯      U+222f      e2 88 af     SURFACE INTEGRAL                          Mathematical Operators                 Math Symbol
       U+00a0      c2 a0        NO-BREAK SPACE                            Latin-1 Supplement                     Space
E      U+0045      45           LATIN CAPITAL LETTER E                    Basic Latin                            Uppercase Letter
⋅      U+22c5      e2 8b 85     DOT OPERATOR                              Mathematical Operators                 Math Symbol
d      U+0064      64           LATIN SMALL LETTER D                      Basic Latin                            Lowercase Letter
a      U+0061      61           LATIN SMALL LETTER A                      Basic Latin                            Lowercase Letter
       U+0020      20           SPACE                                     Basic Latin                            Space
🙌     U+1f64c     f0 9f 99 8c  PERSON RAISING BOTH HANDS IN CELEBRATION  Emoticons                              Other Symbol
🏽     U+1f3fd     f0 9f 8f bd  EMOJI MODIFIER FITZPATRICK TYPE-4         Miscellaneous Symbols and Pictographs  Modifier Symbol
```

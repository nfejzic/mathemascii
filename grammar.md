# AsciiMath grammar

This is the grammar for the [AsciiMath syntax](http://asciimath.org/#syntax).
It is a reference and documentation for this crate.

Apart from custom symbols, AsciiMath allows TeX alternatives for some. This
crate aims to implement the AsciiMath definitions faithfully.

## Symbols

### Operation symbols

| Symbol | TeX alternative     |
| ------ | ------------------- |
| +      | none                |
| -      | none                |
| \*     | cdot                |
| \*\*   | ast                 |
| \*\*\* | star                |
| //     | none                |
| \\     | backslash, setminus |
| xx     | times               |
| -:     | div                 |
| \|><   | ltimes              |
| ><\|   | rtimes              |
| \|><\| | bowtie              |
| @      | circ                |
| o+     | oplus               |
| ox     | otimes              |
| o.     | odot                |
| sum    | none                |
| prod   | none                |
| ^^     | wedge               |
| ^^^    | biwedge             |
| vv     | vee                 |
| vvv    | bigvee              |
| nn     | cap                 |
| nnn    | bigcap              |
| uu     | cup                 |
| uuu    | bigcup              |

### Miscellaneous Symbols

| Symbol     | TeX alternative |
| ---------- | --------------- |
| 2/3        | frac{2}{3}      |
| 2^3        | none            |
| sqrt x     | none            |
| root(3)(x) | none            |
| int        | none            |
| oint       | none            |
| del        | partial         |
| grad       | nabla           |
| +-         | pm              |
| O/         | emptyset        |
| oo         | infty           |
| aleph      | none            |
| :.         | therefore       |
| :'         | because         |
| \|...\|    | \|ldots\|       |
| \|cdots\|  | none            |
| vdots      | none            |
| ddots      | none            |
| \|\\\|     | none            |
| \|quad\|   | none            |
| /\_        | angle           |
| frown      | none            |
| /\_\\      | triangle        |
| diamond    | none            |
| square     | none            |
| \|\_\_     | lfloor          |
| \_\_\|     | rfloor          |
| \|~        | lceiling        |
| ~\|        | rceiling        |
| CC         | none            |
| NN         | none            |
| QQ         | none            |
| RR         | none            |
| ZZ         | none            |
| "hi"       | text(hi)        |

### Relation symbols

| Symbol | TeX alternative |
| ------ | --------------- |
| =      | none            |
| !=     | ne              |
| <      | lt              |
| >      | gt              |
| <=     | le              |
| >=     | ge              |
| mlt    | ll              |
| mgt    | gg              |
| -<     | prec            |
| -<=    | preceq          |
| >-     | succ            |
| >-=    | succeq          |
| in     | none            |
| !in    | notin           |
| sub    | subset          |
| sup    | supset          |
| sube   | subseteq        |
| supe   | supseteq        |
| -=     | equiv           |
| ~=     | cong            |
| ~~     | approx          |
| prop   | propto          |

### Logical symbols

| Symbol | TeX alternative |
| ------ | --------------- |
| and    | none            |
| or     | none            |
| not    | neg             |
| =>     | implies         |
| if     | none            |
| <=>    | iff             |
| AA     | forall          |
| EE     | exists          |
| \_\|\_ | bot             |
| TT     | top             |
| \|--   | vdash           |
| \|==   | models          |

### Grouping brackets

| Symbol     | TeX alternative |
| ---------- | --------------- |
| (          | none            |
| )          | none            |
| [          | none            |
| ]          | none            |
| {          | none            |
| }          | none            |
| (:         | langle          |
| :)         | rangle          |
| <<         | none            |
| >>         | none            |
| {: x )     | none            |
| ( x :}     | none            |
| abs(x)     | none            |
| floor(x)   | none            |
| ceil(x)    | none            |
| norm(vecx) | none            |

### Arrows

| Symbol | TeX alternative       |
| ------ | --------------------- |
| uarr   | uparrow               |
| darr   | downarrow             |
| rarr   | rightarrow            |
| ->     | to                    |
| ->-    | rightarrowtail        |
| ->>    | twoheadrightarrow     |
| >->>   | twoheadrightarrowtail |
| \|->   | mapsto                |
| larr   | leftarrow             |
| harr   | leftrightarrow        |
| rArr   | Rightarrow            |
| lArr   | Leftarrow             |
| hArr   | Leftrightarrow        |

### Accents

| Symbol         | TeX alternative |
| -------------- | --------------- |
| hat x          | none            |
| bar x          | overline x      |
| ul x           | underline x     |
| vec x          | none            |
| tilde x        | none            |
| dot x          | none            |
| ddot x         | none            |
| overset(x)(=)  | overset(x)(=)   |
| underset(x)(=) | none            |
| ubrace(1+2)    | underbrace(1+2) |
| obrace(1+2)    | overbrace(1+2)  |
| color(red)(x)  | none            |
| cancel(x)      | none            |

### Greek Letters

| Symbol     | TeX alternative |
| ---------- | --------------- |
| alpha      | none            |
| beta       | none            |
| gamma      | none            |
| Gamma      | none            |
| delta      | none            |
| epsilon    | none            |
| varepsilon | none            |
| zeta       | none            |
| eta        | none            |
| theta      | none            |
| vartheta   | none            |
| iota       | none            |
| kappa      | none            |
| lambda     | none            |
| mu         | none            |
| nu         | none            |
| xi         | none            |
| Xi         | none            |
| pi         | none            |
| Pi         | none            |
| rho        | none            |
| sigma      | none            |
| tau        | none            |
| upsilon    | none            |
| phi        | none            |
| Phi        | none            |
| varphi     | none            |
| chi        | none            |
| psi        | none            |
| Psi        | none            |
| omega      | none            |
| Omega      | none            |

### Font commands

| Symbol     | TeX alternative |
| ---------- | --------------- |
| bb "text"  | mathbf "text"   |
| bbb "text" | mathbb "text"   |
| cc "text"  | mathcal "text"  |
| tt "text"  | mathtt "text"   |
| fr "text"  | mathfrak "text" |
| sf "text"  | mathsf "text"   |

### Standard functions

`functions = sin | cos | tan | sec | csc | cot | arcsin | arccos | arctan | sinh | cosh | tanh | sech |
csch | coth | exp | log | ln | det | dim | mod | gcd | lcm | lub | glb | min | max | f | g`

### Special cases

Matrices: `[[a,b],[c,d]]` -> `[a,b]` is a single row
Column vectors: `((a),(b))`
Augmented matrices: `[[a,b,|,c],[d,e,|,f]]` -> `[a,b,|,c]` is a single row,
third column is separated from the rest with a single continuous vertical bar.

Matrices for layout: `{(2x,+,17y,=,23),(x,-,y,=,5):}`

Complex subscripts: `lim_(N->oo) sum_(i=0)^N`

Subscripts must come before superscripts: `int_0^1 f(x)dx`

Derivatives: `f'(x) = dy/dx`

For variables other than `x | y | z | t` grouping is needed: `(dq)/(dp)`

Overbraces and underbraces: `ubrace(1+2+3+4)_("4 terms")`

## Grammar

Backus-Naur form of the grammar:

```EBNF
v ::= [A-Za-z] | greek_letters | numbers | other_constant_symbols ;
u ::= sqrt | text | bb | other_unary_symbols_for_font_commands ;
b ::= frac | root | stackrel | other_binary_symbols ;
l ::= "(" | "[" | "{" | "(:" | "{:" | other_left_brackets ;
r ::= ")" | "]" | "}" | ":)" | ":}" | other_right_brackets ;
S ::= v | lEr | uS | bSS ;            (* Simple expression *)
I ::= S_S | S^S | S_S^S | S ;         (* Intermediate expression *)
E ::= IE | I/I ;                      (* Expression *)
```

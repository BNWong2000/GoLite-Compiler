# Language Specification:
This language is intended to be a simplified subset of Google's Golang. Specification details are all derived from the [Golang spec](https://go.dev/ref/spec) and the [GoLite spec](https://www.cs.mcgill.ca/~cs520/2019/project/Milestone1_Specifications.pdf), and were copied into this document, in order to clarify the ways in which I am utilizing it for this project. 

### Lexical Specification:

#### Identifiers:
Identifiers are non-keywords that start with either an alphabetical character or a `_` character, followed by zero or more alphanumeric or `_` characters.

#### Literals:
Types of literals supported include:
Base 10 Integer Literals,
Floats with either or both of the integral and decimal parts included.
Char Literals, including the following escapes:
```
\a \b \f \n \r
\t \v \\ \’ \"
```
Interpreted String Literals (i.e., C-Style string literals)

#### Keywords:
```
break        default      func         interface    select
case         defer        go           map          struct
chan         else         goto         package      switch
const        fallthrough  if           range        type
continue     for          import       return       var
```

#### Operators and Punctuation:
```
+    &     +=    &=     &&    ==    !=    (    )
-    |     -=    |=     ||    <     <=    [    ]
*    ^     *=    ^=     <-    >     >=    {    }
/    <<    /=    <<=    ++    =     :=    ,    ;
%    >>    %=    >>=    --    !     ...   .    :
&^   &^=   ~
```

#### Comments:
Support for single line `//` and block `/* */` comments
- Block comments may not nest. 

#### Semicolons:


### Grammar:

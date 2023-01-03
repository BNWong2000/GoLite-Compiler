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
\t \v \\ \' \"
```
Chars will be 7-bit ASCII standard (non-extended ascii)
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
&^   &^=   ~     "      '
```

#### Comments:
Support for single line `//` and block `/* */` comments
- Block comments may not nest. 

#### Semicolons:
Follows part of the Go standard for semicolons. Specifically, semicolons are implied when:
> When the input is broken into tokens, a semicolon is automatically inserted into the token stream immediately after a line's final token if that token is
> - an identifier
> - an integer, floating-point, imaginary, rune, or string literal
> - one of the keywords break, continue, fallthrough, or return
> - one of the operators and punctuation ++, --, ), ], or }


### Grammar:
```
program         = "package" identifier ';' TopLevelDecl .

TopLevelDecl    = Declaration | FunctionDecl | MethodDecl .

Declaration     = ConstDecl | TypeDecl | VarDecl .

ConstDecl       = "const" ( ConstSpec | "(" { ConstSpec ";" } ")" ) .

ConstSpec       = IdentifierList [ [ Type ] "=" ExpressionList ] .

IdentifierList  = identifier { "," identifier } .

Type            = TypeName [ TypeArgs ] | TypeLit | "(" Type ")" .
TypeName        = identifier
TypeArgs        = "[" TypeList [ "," ] "]" .
TypeList        = Type { "," Type } .
TypeLit         = ArrayType | StructType | PointerType | FunctionType | InterfaceType |
                  SliceType | MapType | ChannelType .

ArrayType       = "[" Expression "]" Type .

StructType      = "struct" "{" { FieldDecl ";" } "}" .
FieldDecl       = (IdentifierList Type | EmbeddedField) [ string_lit ] .
EmbeddedField   = [ "*" ] TypeName [ TypeArgs ] .

PointerType     = "*" Type .

FunctionType    = "func" Signature .
Signature       = Parameters [ Result ] .
Result          = Parameters | Type .
Parameters      = "(" [ ParameterList [ "," ] ] ")" .
ParameterList   = ParameterDecl { "," ParameterDecl } .
ParameterDecl   = [ IdentifierList ] [ "..." ] Type .

InterfaceType   = "interface" "{" { InterfaceElem ";" } "}" .
InterfaceElem   = MethodElem | TypeElem .
MethodElem      = identifier Signature .
TypeElem        = TypeTerm { "|" TypeTerm } .
TypeTerm        = Type | UnderlyingType .
UnderlyingType  = "~" Type .

SliceType       = "[" "]" Type .

MapType         = "map" "[" Type "]" Type .

ChannelType     = ( "chan" | "chan" "<-" | "<-" "chan" ) Type .

ExpressionList  = Expression { "," Expression } .

Expression      = UnaryExpr | Expression binary_op Expression .
UnaryExpr       = PrimaryExpr | unary_op UnaryExpr .

binary_op       = "||" | "&&" | rel_op | add_op | mul_op .
rel_op          = "==" | "!=" | "<" | "<=" | ">" | ">=" .
add_op          = "+" | "-" | "|" | "^" .
mul_op          = "*" | "/" | "%" | "<<" | ">>" | "&" | "&^" .

unary_op        = "+" | "-" | "!" | "^" | "*" | "&" | "<-" .

PrimaryExpr     = Operand | Conversion | MethodExpr | PrimaryExpr Selector |
	              PrimaryExpr Index | PrimaryExpr Slice | PrimaryExpr TypeAssertion |
	              PrimaryExpr Arguments .

Selector        = "." identifier .
Index           = "[" Expression "]" .
Slice           = "[" [ Expression ] ":" [ Expression ] "]" |
                  "[" [ Expression ] ":" Expression ":" Expression "]" .
TypeAssertion   = "." "(" Type ")" .
Arguments       = "(" [ ( ExpressionList | Type [ "," ExpressionList ] ) [ "..." ] [ "," ] ] ")" .

Operand         = Literal | identifier [ TypeArgs ] | "(" Expression ")" .
Literal         = BasicLit | CompositeLit | FunctionLit .
BasicLit        = int_lit | float_lit | rune_lit | string_lit .

CompositeLit    = LiteralType LiteralValue .
LiteralType     = StructType | ArrayType | "[" "..." "]" Type |
                  SliceType | MapType | TypeName [ TypeArgs ] .
LiteralValue    = "{" [ ElementList [ "," ] ] "}" .
ElementList     = KeyedElement { "," KeyedElement } .
KeyedElement    = [ Key ":" ] Element .
Key             = FieldName | Expression | LiteralValue .
FieldName       = identifier .
Element         = Expression | LiteralValue .

FunctionLit     = "func" Signature Block .

Conversion      = Type "(" Expression [ "," ] ")" .

MethodExpr      = Type "." identifier .

TypeDecl        = "type" ( TypeSpec | "(" { TypeSpec ";" } ")" ) .
TypeSpec        = AliasDecl | TypeDef .
AliasDecl       = identifier "=" Type .
TypeDef         = identifier [ TypeParameters ] Type .

TypeParameters  = "[" TypeParamList [ "," ] "]" .
TypeParamList   = TypeParamDecl { "," TypeParamDecl } .
TypeParamDecl   = IdentifierList TypeConstraint .

TypeConstraint  = TypeElem .

VarDecl         = "var" ( VarSpec | "(" { VarSpec ";" } ")" ) .
VarSpec         = IdentifierList ( Type [ "=" ExpressionList ] | "=" ExpressionList ) .

FunctionDecl    = "func" identifier [ TypeParameters ] Signature [ Block ] .

Block           = "{" StatementList "}" .
StatementList   = { Statement ";" } .

Statement       = Declaration | LabeledStmt | SimpleStmt | GoStmt | ReturnStmt | 
                  BreakStmt | ContinueStmt | GotoStmt | "fallthrough" | Block | 
                  IfStmt | SwitchStmt | SelectStmt | ForStmt | DeferStmt .

SimpleStmt      = EmptyStmt | Expression | SendStmt | IncDecStmt | Assignment | ShortVarDecl .

EmptyStmt       = .

LabeledStmt     = identifier ":" Statement .

GoStmt          = "go" Expression .
ReturnStmt      = "return" [ ExpressionList ] .
BreakStmt       = "break" [ identifier ] .
ContinueStmt    = "continue" [ identifier ] .
GotoStmt        = "goto" identifier .
IfStmt          = "if" [ SimpleStmt ";" ] Expression Block [ "else" ( IfStmt | Block ) ] .
SwitchStmt      = ExprSwitchStmt | TypeSwitchStmt .

ExprSwitchStmt  = "switch" [ SimpleStmt ";" ] [ Expression ] "{" { ExprCaseClause } "}" .
ExprCaseClause  = ExprSwitchCase ":" StatementList .
ExprSwitchCase  = "case" ExpressionList | "default" .

TypeSwitchStmt  = "switch" [ SimpleStmt ";" ] TypeSwitchGuard "{" { TypeCaseClause } "}" .
TypeSwitchGuard = [ identifier ":=" ] PrimaryExpr "." "(" "type" ")" .
TypeCaseClause  = TypeSwitchCase ":" StatementList .
TypeSwitchCase  = "case" TypeList | "default" .

SelectStmt      = "select" "{" { CommClause } "}" .
CommClause      = CommCase ":" StatementList .
CommCase        = "case" ( SendStmt | RecvStmt ) | "default" .
RecvStmt        = [ ExpressionList "=" | IdentifierList ":=" ] Expression .

SendStmt        = Expression "<-" Expression .

ForStmt         = "for" [ Expression | ForClause | RangeClause ] Block .

ForClause       = [ SimpleStmt ] ";" [ Condition ] ";" [ SimpleStmt ] .
RangeClause     = [ ExpressionList "=" | IdentifierList ":=" ] "range" Expression .

DeferStmt       = "defer" Expression .

IncDecStmt      = Expression ( "++" | "--" ) .

Assignment      = ExpressionList assign_op ExpressionList .

assign_op       = [ add_op | mul_op ] "=" .

ShortVarDecl    = IdentifierList ":=" ExpressionList .

MethodDecl      = "func" Receiver identifier Signature [ Block ] .
Receiver        = Parameters .
```

Semantic checks:
Goto/break/continue labels vs other identifiers
Type checking
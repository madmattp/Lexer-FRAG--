# FRAG-- LEXER

## Gramática:
Program → StatementList

Modifier → "FLOAT" | "BOOLEAN" | "INTEGER" | "PARAM" | "VECTOR" "[" "NUM" "]" | "TYPE" "(" "ID" ")"

Id_List → "ID" | "ID" "," Id_List

ModifierList → Modifier | ModifierList Modifier

NameDecl → ModifierList Id_List

Fragment → "FRAGMENT" "ID" ";" StatementList "ENDFRAGMENT" | "FRAGMENT" StatementList "ENDFRAGMENT" | "FRAGMENT" ModifierList "ID" ";" StatementList "ENDFRAGMENT" | "FRAGMENT" "ID" ";" "LITERAL" "ENDFRAGMENT" | "FRAGMENT" ModifierList "ID" ";" "NUM" "ENDFRAGMENT"

StatementList → Statement StatementList | Statement

Statement → NameDecl ";" | Destiny "=" Expression ";" | Calfunc ";" | Fragment | "IF" "(" Expression ")" StatementList "ELSE" StatementList | "IF" "(" Expression ")" StatementList | "READ" "(" ExpressionList ")" ";" |  "WHILE" "(" Expression ")" StatementList | "SELECT" "(" Expression ")" "FRAGMENT" CaseBlock "ENDFRAGMENT" | "BREAK" ";" | "WRITE" "(" ExpressionList ")" ";"

Destiny → "ID" | "ID" "[" Expression "]" | Name

CaseBlock → "CASE" Expression ":" StatementList CaseBlock | "DEFAULT" ":" StatementList CaseBlock

ExpressionList → Expression | ExpressionList "," Expression

Expression → Primary | UnaryOp Expression | Expression BinOp Expression

Primary → Name | Calfunc | "(" Expression ")" | "ID" "(" ExpressionList ")" | "TRUE" | "FALSE" | "LITERAL"

Name → "ID" | "ID" "[" Expression "]" | Name "." Name

UnaryOp → "-" | "!" | "+"

BinOp → "==" | "<" | "<=" | ">=" | ">" | "!=" | "+" | "-" | "*" | "/" | "&&" | "||"

Calfunc → "ID" "(" ExpressionList ")"

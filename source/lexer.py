import re
from sys import argv

# Expressões regulares
token_exprs = [
    (r'FLOAT', 'KW_FLOAT'),
    (r'BOOLEAN', 'KW_BOOL'),
    (r'INTEGER', 'KW_INT'),
    (r'PARAM', 'KW_PARAM'),
    (r'VECTOR', 'KW_VECTOR'),
    (r'TYPE', 'KW_TYPE'),
    (r'FRAGMENT', 'KW_FRAG'),
    (r'ENDFRAGMENT', 'KW_ENDFRAG'),
    (r'IF', 'KW_IF'),
    (r'ELSE', 'KW_ELSE'),
    (r'WHILE', 'KW_WHILE'),
    (r'READ', 'KW_READ'),
    (r'WRITE', 'KW_WRITE'),
    (r'SELECT', 'KW_SELECT'),
    (r'BREAK', 'KW_BREAK'),
    (r'CASE', 'KW_CASE'),
    (r'DEFAULT', 'KW_DEFAULT'),
    (r'TRUE', 'KW_TRUE'),
    (r'FALSE', 'KW_FALSE'),
    (r'==', 'OP_EQ'),
    (r'<=', 'OP_LE'),
    (r'>=', 'OP_GE'),
    (r'!=', 'OP_NE'),
    (r'&&', 'OP_AND'),
    (r'\|\|', 'OP_OR'),
    (r'\|', 'OP_BITOR'),
    (r'%', 'OP_MOD'),
    (r'&', 'OP_BITAND'),
    (r'!', 'OP_NOT'),
    (r'=', 'OP_ASSIGN'),
    (r'<', 'OP_LT'),
    (r'>', 'OP_GT'),
    (r'\+', 'OP_PLUS'),
    (r'-', 'OP_MINUS'),
    (r'\*', 'OP_TIMES'),
    (r'/', 'OP_DIVIDE'),
    (r'\(', 'L_PAREN'),
    (r'\)', 'R_PAREN'),
    (r'\{', 'L_BRACE'),
    (r'\}', 'R_BRACE'),
    (r'\[', 'L_BRACKET'),
    (r'\]', 'R_BRACKET'),
    (r';', 'SEMICOLON'),
    (r',', 'COMMA'),
    (r'\.', 'DOT'),
    (r'[a-zA-Z_][a-zA-Z0-9_]*', 'ID'),
    (r'[0-9]+', 'NUM'),
    (r"'[^']*'", 'CHAR_LITERAL'),
    (r'"[^"]*"', 'STRING'),
    (r'/\*([^*]|\*[^/])*\*/', 'COMMENT'),
    (r'[ \t\n\r]+', 'WHITESPACE')
]

# Função de análise léxica
def lex(characters):
    pos = 0
    tokens = []
    while pos < len(characters):
        match = None
        for token_expr in token_exprs:
            pattern, tag = token_expr
            regex = re.compile(pattern)
            match = regex.match(characters, pos)
            if match:
                text = match.group(0)
                if tag != 'WHITESPACE' and tag != 'COMMENT':  # Ignorar espaços e comentários
                    token = (tag, text)
                    tokens.append(token)
                break
        if not match:
            raise SyntaxError(f'Erro léxico: {characters[pos]}')
        else:
            pos = match.end(0)
    tokens.append(('EOF', 'EOF'))
    return tokens


def main():
    if (len(argv) < 2) or (len(argv) > 2):
        print(f"Usage: {argv[0]} [file.frag]")
        exit()
    else:
        file = open(argv[1], "r", encoding="utf8").read()
        print(lex(file))

if __name__ == "__main__":
    main()

    
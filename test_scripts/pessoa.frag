FRAGMENT Pessoa;
    integer idade;
    float peso;
    boolean anoNascimento;
    boolean solteiro;
ENDFRAGMENT
FRAGMENT integer tamanhoVetor;
    10
ENDFRAGMENT
vector[10] TYPE (Pessoa) pessoa;
integer i;
i = 0;
WHILE(i < tamanhoVetor) FRAGMENT
    pessoa[i].idade = (i * 50)/50;
    pessoa[i].peso = 96 + i/2;
    pessoa[i].anoNascimento = 1990 + 1;
    pessoa[i].solteiro = (i/2) == 0;
    i = i + 1;
ENDFRAGMENT
i = 0;
WHILE(i < tamanhoVetor) FRAGMENT
    WRITE("Pessoa ", i, "\n");
    WRITE("Peso: ", pessoa[i].peso, "\n");
    WRITE("Idade: ", pessoa[i].idade, "\n");
    WRITE("Ano de nascimento: ", pessoa[i].anoNascimento, "\n");
    IF (pessoa[i].solteiro)
        WRITE("Está solteiro: Sim \n");
    ELSE
        WRITE("Não está solteiro: Sim \n");
    WRITE("Idade + Peso = ", pessoa[i].idade + pessoa[i].peso, "\n");
    i = i + 1;
ENDFRAGMENT
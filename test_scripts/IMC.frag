FRAGMENT Imc;
    float altura;
    float peso;
    float imc;
    boolean masculino;
ENDFRAGMENT

TYPE (Imc) jo;
jo.altura = 1, 7;
jo.peso = 70, 0;
jo.masculino = TRUE;
jo.imc = jo.peso/(jo.altura*jo.altura);
IF (jo.masculino)
    IF (jo.imc < 20, 7 || jo.imc > 26, 4)
        WRITE("Não está no peso ideal. Seu IMC é ", jo.imc);
    ELSE
        WRITE("Está no peso ideal. Seu IMC é ", jo.imc);
ELSE
    WRITE("Pessoa do sexo feminino");
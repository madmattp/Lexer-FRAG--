FRAGMENT areaTriangulo;
    float base;
    float altura;
    float A;
ENDFRAGMENT
TYPE(areaTriangulo) area;
area.base = 2;
area.altura = 3;
area.A = area.base * area.altura/2;
WRITE("Área do triângulo = ", area.A);
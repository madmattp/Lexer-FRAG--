integer vetor[10] jo;
integer k;
WHILE (k < 10) FRAGMENT
    jo[k] = k;
    k = k * (k + 1) * 2;
ENDFRAGMENT

k = 0;
WHILE (k < 10) FRAGMENT
    WRITE("[", k, "] = ", jo[k], "\n");
    k = k + 1;
ENDFRAGMENT
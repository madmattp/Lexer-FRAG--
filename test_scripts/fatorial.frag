FRAGMENT INTEGER Fatorial;
    INTEGER PARAM n;
    IF (n != 1 && n > 0) FRAGMENT
        Fatorial = n * Fatorial(n - 1);
        BREAK;
    ENDFRAGMENT ELSE
        Fatorial = n * Fatorial(n - 1);
ENDFRAGMENT
INTEGER numFatorial;
numFatorial = 10;
WRITE("Fatorial de", numFatorial, ":", Fatorial(numFatorial));
    

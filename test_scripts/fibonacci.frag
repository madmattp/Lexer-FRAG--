FRAGMENT integer Fibonacci;
    integer param num;
    IF (num == 1 || num == 2)
        Fibonacci = 1;
        BREAK;
    ELSE
        Fibonacci = Fibonacci(n - 1) + Fibonacci(n - 2);
ENDFRAGMENT
WRITE("Fibonacci:", Fibonacci(20));
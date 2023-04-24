## Maquina de turing deterministica

Insira um arquivo nesse formato para utilizar a maquina:
```
q0;q1;q2;q3 (conjunto de estados)
a;b;c;d;e;f;g (alfabeto)
a;b;c;d;e;f;g;X;Y;Y (alfabeto fita)
q0 (estado inicial)
q2;q3 (estados finais)
$ (simbolo branco)
(q0,a)->(q1,a,d);(q0,a)->(q1,a,d);(q0,a)->(q1,a,d) (função transição)
```

## PS:
Código precisa de algumas refatorações mas pelo menos é bem claro saber
onde é preciso refatorar.

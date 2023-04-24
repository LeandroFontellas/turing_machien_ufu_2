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

## Algoritmo
A simulação tem duas entidades principais a própria maquina de turing, a fita
utilizada. Ambas são inicializadas no inicio através da descrição contida nos
arquivos .txt. A maquina de turing tem a setupla ( Q, Σ, Γ, δ, q0, B, F) onde as fun
ções de transição ficam armazenadas em uma tabela hash. A fita é onde a MT irá
trabalhar escrevendo sobre ela e andando com a cabeça para a esquerda ou direita.

## PS:
Código precisa de algumas refatorações mas pelo menos é bem claro saber
onde é preciso refatorar.

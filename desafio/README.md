DESAFIO 

Imagine que você está desenvolvendo um sistema de atendimento para um banco. Nesse sistema, os clientes chegam e aguardam na fila para serem atendidos na ordem de chegada (modelo FIFO – First In, First Out). Como parte desse sistema, você precisa implementar a fila do zero. Essa implementação será utilizada para controlar o fluxo de atendimento dos clientes, garantindo que o primeiro a chegar seja o primeiro a ser atendido.

Fila Genérica em Rust

Esta é uma implementação de uma fila genérica (FIFO) em Rust, permitindo armazenar e manipular elementos de qualquer tipo.

Funcionalidades
•⁠  ⁠Adicionar Elemento ⁠ enqueue(elemento) ⁠ - Insere um novo elemento no final da fila.
•⁠  ⁠Remover Elemento: ⁠ dequeue() ⁠ - Remove e retorna o primeiro elemento da fila.
•⁠  ⁠Visualizar Próximo Elemento: ⁠ peek() ⁠ - Retorna o próximo elemento sem removê-lo.
•⁠  ⁠Tamanho da Fila: ⁠ len() ⁠ - Retorna a quantidade de elementos presentes na fila.

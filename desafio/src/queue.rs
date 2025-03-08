// Define a estrutura `Node`, que representa um nó da fila.
// Cada nó armazena um valor e um ponteiro opcional para o próximo nó.
pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

// Define a estrutura `Queue`, que representa a fila propriamente dita.
// Ela mantém um ponteiro para o primeiro nó (`head`), um ponteiro cru para o último nó (`tail`),
// e um contador de elementos (`len`).
pub struct Queue<T> {
    head: Option<Box<Node<T>>>, // Primeiro elemento da fila.
    tail: *mut Node<T>,         // Último elemento da fila (usando ponteiro cru).
    len: usize,                 // Número de elementos na fila.
}

impl<T> Queue<T> {
    // Cria uma nova fila vazia.
    pub fn new() -> Self {
        Queue {
            head: None,               // Inicialmente, a fila está vazia.
            tail: std::ptr::null_mut(), // O ponteiro para o último nó é nulo.
            len: 0,                   // O tamanho da fila é zero.
        }
    }

    // Adiciona um elemento ao final da fila.
    pub fn enqueue(&mut self, elem: T) {
        let new_node = Box::new(Node {
            value: elem,
            next: None, // O novo nó não tem próximo elemento ainda.
        });

        let raw_node = Box::into_raw(new_node); // Converte o nó em um ponteiro cru.

        unsafe {
            if self.tail.is_null() {
                // Se a fila estiver vazia, o novo nó se torna o primeiro.
                self.head = Some(Box::from_raw(raw_node));
                self.tail = raw_node;
            } else {
                // Caso contrário, o último nó atual passa a apontar para o novo nó.
                (*self.tail).next = Some(Box::from_raw(raw_node));
                self.tail = raw_node; // Atualiza o ponteiro do último nó.
            }
        }
        self.len += 1; // Incrementa o tamanho da fila.
    }

    // Remove e retorna o primeiro elemento da fila.
    pub fn dequeue(&mut self) -> Option<T> {
        self.head.take().map(|mut node| {
            self.head = node.next.take(); // Atualiza o ponteiro `head` para o próximo nó.
            if self.head.is_none() {
                self.tail = std::ptr::null_mut(); // Se a fila ficou vazia, zera o ponteiro `tail`.
            }
            self.len -= 1; // Decrementa o tamanho da fila.
            node.value // Retorna o valor do nó removido.
        })
    }

    // Retorna uma referência para o primeiro elemento da fila, sem removê-lo.
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    // Retorna o número de elementos na fila.
    pub fn len(&self) -> usize {
        self.len
    }
}

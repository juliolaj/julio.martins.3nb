// Define um módulo de testes que só será compilado e executado durante testes.
#[cfg(test)]
mod tests {
    // Importa a estrutura `Queue` do módulo `fila_generica::queue`.
    use fila_generica::queue::Queue;

    // Testa a funcionalidade de enfileirar (enqueue) e desenfileirar (dequeue).
    #[test]
    fn test_enqueue_dequeue() {
        let mut fila = Queue::new();
        fila.enqueue(10);
        fila.enqueue(20);
        
        // Verifica se os elementos são removidos na ordem correta.
        assert_eq!(fila.dequeue(), Some(10));
        assert_eq!(fila.dequeue(), Some(20));

        // A fila deve estar vazia após remover todos os elementos.
        assert_eq!(fila.dequeue(), None);
    }

    // Testa a funcionalidade de visualizar o primeiro elemento da fila sem removê-lo (peek).
    #[test]
    fn test_peek() {
        let mut fila = Queue::new();
        fila.enqueue(100);

        // O primeiro elemento deve ser 100.
        assert_eq!(fila.peek(), Some(&100));

        // Após remover o elemento, a fila deve ficar vazia.
        fila.dequeue();
        assert_eq!(fila.peek(), None);
    }

    // Testa se o tamanho da fila é atualizado corretamente.
    #[test]
    fn test_len() {
        let mut fila = Queue::new();

        // A fila começa vazia.
        assert_eq!(fila.len(), 0);

        // Adiciona dois elementos e verifica o tamanho.
        fila.enqueue(5);
        fila.enqueue(15);
        assert_eq!(fila.len(), 2);

        // Remove um elemento e verifica o novo tamanho.
        fila.dequeue();
        assert_eq!(fila.len(), 1);
    }
}

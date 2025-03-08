// Declara o módulo 'queue', que deve estar definido em outro arquivo ou diretório.
mod queue;

// Importa a estrutura 'Queue' do módulo 'queue'.
use queue::Queue;

// Função principal do programa.
fn main() {
    // Cria uma nova fila vazia.
    let mut fila = Queue::new();

    // Adiciona elementos à fila.
    fila.enqueue(10);
    fila.enqueue(20);

    // Exibe o primeiro elemento da fila sem removê-lo.
    println!("Primeiro elemento: {:?}", fila.peek());

    // Remove e exibe o primeiro elemento da fila.
    println!("Removido: {:?}", fila.dequeue());

    // Exibe o tamanho atual da fila.
    println!("Tamanho atual: {}", fila.len());
}

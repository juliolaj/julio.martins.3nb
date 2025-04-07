// Teste Árvore Binária -

#[cfg(test)]
mod testes_arvore {
    use crate::Arvore;

    /* Arvore
    Importe aqui*/
    
    
    #[test]
    fn teste_arvore_nova_e_vazia() {
        // T1: Criar uma nova árvore e verificar se está vazia
        let arvore = Arvore::nova();
        assert!(arvore.esta_vazia());
    }
    
    #[test]
    fn teste_inserir_e_buscar_elementos() {
        // T2: Inserir elementos e verificar se estão presentes na árvore
        let mut arvore = Arvore::nova();
        
        // Inserir alguns números
        arvore.inserir(10);
        arvore.inserir(5);
        arvore.inserir(15);
        
        // Verificar se os números inseridos estão na árvore
        assert!(arvore.buscar(10));
        assert!(arvore.buscar(5));
        assert!(arvore.buscar(15));
        
        // Verificar que um valor não inserido não está na árvore
        assert!(!arvore.buscar(20));
        
        // A árvore não deve estar vazia após inserções
        assert!(!arvore.esta_vazia());
    }
}

#[derive(Debug)]
pub struct Nodo {
    pub valor: i32,
    pub esquerda: Option<Box<Nodo>>,
    pub direita: Option<Box<Nodo>>,
}

pub struct Arvore {
    pub raiz: Option<Box<Nodo>>,
}

impl Arvore {
    // Criar uma nova árvore vazia
    pub fn nova() -> Self {
        Arvore { raiz: None }
    }

    // Verificar se a árvore está vazia
    pub fn esta_vazia(&self) -> bool {
        self.raiz.is_none()
    }

    // Inserir um valor na árvore
    pub fn inserir(&mut self, valor: i32) {
        let novo_nodo = Box::new(Nodo { valor, esquerda: None, direita: None });
        if self.raiz.is_none() {
            self.raiz = Some(novo_nodo);
        } else {
            Arvore::inserir_nodo(self.raiz.as_mut().unwrap(), novo_nodo);
        }
    }

    fn inserir_nodo(atual: &mut Box<Nodo>, novo_nodo: Box<Nodo>) {
        if novo_nodo.valor < atual.valor {
            if atual.esquerda.is_none() {
                atual.esquerda = Some(novo_nodo);
            } else {
                Arvore::inserir_nodo(atual.esquerda.as_mut().unwrap(), novo_nodo);
            }
        } else {
            if atual.direita.is_none() {
                atual.direita = Some(novo_nodo);
            } else {
                Arvore::inserir_nodo(atual.direita.as_mut().unwrap(), novo_nodo);
            }
        }
    }

    // Buscar um valor na árvore
    pub fn buscar(&self, valor: i32) -> bool {
        Arvore::buscar_nodo(self.raiz.as_ref(), valor)
    }

    fn buscar_nodo(atual: Option<&Box<Nodo>>, valor: i32) -> bool {
        match atual {
            Some(nodo) => {
                if nodo.valor == valor {
                    true
                } else if valor < nodo.valor {
                    Arvore::buscar_nodo(nodo.esquerda.as_ref(), valor)
                } else {
                    Arvore::buscar_nodo(nodo.direita.as_ref(), valor)
                }
            }
            None => false,
        }
    }
}
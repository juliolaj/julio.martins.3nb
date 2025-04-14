use petgraph::graph::{Graph};
use petgraph::visit::Dfs;

pub struct MyGraph {
    graph: Graph<&'static str, ()>,
}

impl MyGraph {
    pub fn new() -> Self {
        let mut meu_grafo = Graph::new();

        // Criando os pontos
        let a = meu_grafo.add_node("1");
        let b = meu_grafo.add_node("2");
        let c = meu_grafo.add_node("3");
        let d = meu_grafo.add_node("4");
        let e = meu_grafo.add_node("5");
        let f = meu_grafo.add_node("6");

        // Ligando os pontos com linhas 
        meu_grafo.add_edge(a, b, ());
        meu_grafo.add_edge(a, c, ());
        meu_grafo.add_edge(b, d, ());
        meu_grafo.add_edge(c, e, ());
        meu_grafo.add_edge(d, f, ());
        meu_grafo.add_edge(e, f, ());

        MyGraph { graph: meu_grafo }
    }

    pub fn dfs_from_node1(&self) -> Vec<&'static str> {
        let mut lista_resultado = Vec::new();

        // Começando a busca a partir do ponto "1"
        let comeca_em = self.graph.node_indices().find(|&i| self.graph[i] == "1").unwrap();
        let mut busca = Dfs::new(&self.graph, comeca_em);

        // Rodando a busca e guardando os pontos visitados
        while let Some(no_atual) = busca.next(&self.graph) {
            lista_resultado.push(self.graph[no_atual]);
        }

        lista_resultado
    }
}

// -----------------------------------------------------------
// TESTES
// -----------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_visits_all_nodes() {
        let meu_teste = MyGraph::new();
        let resultado = meu_teste.dfs_from_node1();

        let mut resultado_ordenado = resultado.clone();
        resultado_ordenado.sort();

        let mut esperado = vec!["1", "2", "3", "4", "5", "6"];
        esperado.sort();

        // Ver se todos os pontos foram visitados
        assert_eq!(resultado_ordenado, esperado, "Todos os nós devem ser visitados");
    }

    #[test]
    fn test_dfs_starts_at_node1() {
        let meu_teste = MyGraph::new();
        let resultado = meu_teste.dfs_from_node1();

        // Ver se começou pelo ponto certo
        assert_eq!(resultado.first(), Some(&"1"), "DFS deve começar pelo nó 1");
    }
}

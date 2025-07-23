use crate::produto::Produto;
use petgraph::graph::{NodeIndex, Graph};
use petgraph::Undirected;
use std::collections::HashMap;

pub struct GrafoProdutos {
    grafo: Graph<String, (), Undirected>,
    indices: HashMap<String, NodeIndex>,
}

impl GrafoProdutos {
    pub fn new(produtos: &[Produto]) -> Self {
        let mut grafo = Graph::<String, (), Undirected>::new_undirected();
        let mut indices = HashMap::new();

        for produto in produtos {
            let idx = grafo.add_node(produto.nome.clone());
            indices.insert(produto.nome.clone(), idx);
        }

        for i in 0..produtos.len() {
            for j in i + 1..produtos.len() {
                if produtos[i].categoria == produtos[j].categoria {
                    let a = indices[&produtos[i].nome];
                    let b = indices[&produtos[j].nome];
                    grafo.add_edge(a, b, ());
                }
            }
        }

        GrafoProdutos { grafo, indices }
    }

    pub fn recomendar(&self, nome: &String) -> Option<Vec<String>> {
        let idx = self.indices.get(nome)?;
        let vizinhos = self.grafo.neighbors(*idx);
        Some(vizinhos.map(|i| self.grafo[i].clone()).collect())
    }
}

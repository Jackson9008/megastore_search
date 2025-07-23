
    use std::collections::HashMap;
use crate::produto::Produto;

pub struct SistemaBusca {
    indice_nome: HashMap<String, Vec<Produto>>,
}

impl SistemaBusca {
    pub fn new(produtos: &[Produto]) -> Self {
        let mut indice_nome: HashMap<String, Vec<Produto>> = HashMap::new();
        for produto in produtos {
            indice_nome.entry(produto.nome.to_lowercase()).or_default().push(produto.clone());
        }
        SistemaBusca { indice_nome }
    }

    pub fn buscar(&self, termo: &str, _categoria: Option<&str>, _marca: Option<&str>) -> Vec<Produto> {
        let termo = termo.to_lowercase();
        let mut resultados = Vec::new();

        for (nome, produtos) in &self.indice_nome {
            if nome.contains(&termo) {
                resultados.extend(produtos.clone());
            }
        }

        resultados
    }
}
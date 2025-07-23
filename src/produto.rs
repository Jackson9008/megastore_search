#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Produto {
    pub id: usize,
    pub nome: String,
    pub marca: String,
    pub categoria: String,
}

impl Produto {
    pub fn carregar_mock() -> Vec<Produto> {
        vec![
            Produto { id: 1, nome: "Notebook Gamer".into(), marca: "Dell".into(), categoria: "Eletrônicos".into() },
            Produto { id: 2, nome: "Smartphone Pro".into(), marca: "Samsung".into(), categoria: "Eletrônicos".into() },
            Produto { id: 3, nome: "Smart TV 50".into(), marca: "LG".into(), categoria: "Eletrônicos".into() },
            Produto { id: 4, nome: "Tênis Esportivo".into(), marca: "Nike".into(), categoria: "Vestuário".into() },
            Produto { id: 5, nome: "Sofá 3 Lugares".into(), marca: "Tok&Stok".into(), categoria: "Decoração".into() },
        ]
    }
}
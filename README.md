# MegaStore Search - Sistema de Recomendação de Produtos com Grafos

## Descrição do Projeto

Este projeto implementa um sistema de recomendação de produtos para a MegaStore, utilizando estruturas de dados avançadas em Rust. O sistema utiliza uma tabela hash para armazenamento eficiente dos produtos e um grafo para modelar as relações entre eles, permitindo recomendações personalizadas baseadas em conexões no grafo.

O objetivo é fornecer uma solução eficiente e escalável para busca e recomendação de produtos, com funcionalidades como filtros por categoria, marca, além de suporte a múltiplos índices de busca.

---

## Funcionalidades

- Armazenamento de produtos em tabelas hash para busca rápida.  
- Implementação de grafo com a crate `petgraph` para modelagem das relações entre produtos.  
- Sistema de recomendação que retorna produtos relacionados com base em critérios configuráveis.  
- Filtros por categoria e marca.
- Testes automatizados cobrindo as principais funcionalidades.  
- Benchmark simples para avaliação de desempenho.

---

## Tecnologias Utilizadas

- Linguagem: Rust (última versão estável)  
- Gerenciamento de dependências: Cargo  
- Biblioteca de grafos: [petgraph](https://crates.io/crates/petgraph)  
- Testes: `cargo test`  
- Benchmark: implementação simples utilizando `std::time::Instant`

---

## Estrutura do Projeto

MegaStore/
├── Cargo.toml
├── src/
│ ├── app.rs
│ ├── busca.rs
│ ├── grafo.rs
│ ├── libs.rs
│ ├── main.rs
│ ├── produto.rs
│ └── utils.rs
├── tests/
│ └── busca_tests.rs
└── README.md

yaml

---

## Como Rodar o Projeto

1. Clone este repositório:

```bash
git clone https://github.com/Jackson9008/MegaStore.git
cd megastore

Compile o projeto:

bash

cargo build --release

Execute o programa:

bash

cargo run --release

Como Usar
No programa principal, você poderá inserir produtos, definir relações entre eles e solicitar recomendações. Exemplo de uso:

Adicionar produto com nome, categoria, marca.

Construir grafo relacionando produtos semelhantes.

Consultar recomendações por nome do produto.

Testes
Para executar os testes automatizados, rode:

bash

cargo test

Benchmark
Há uma rotina simples para medir o tempo de execução das buscas. Execute o programa no modo release para obter melhores resultados:

bash

cargo run --release

Arquitetura
Tabela Hash: Utilizada para indexar produtos por nome, categoria, marca e cliente, garantindo busca rápida.

Grafo: Implementado com petgraph::Graph, conecta produtos relacionados para recomendação.

Módulos:

produto.rs: define a struct Produto.

busca.rs: implementa a lógica de busca e índices.

grafo.rs: implementa a estrutura de grafo e algoritmos de recomendação.

** Próximos Passos

Melhorar a interface GUI 

Integrar sistema de recomendação mais eficiente.

Adicionar persistência de dados com banco de dados.

Implementar algoritmos mais sofisticados de recomendação.

Autor
Jackson Sousa - R.A:96045 - Data Structure Strategy and Implementation
Estudante de Análise e Desenvolvimento de Sistemas – UNIFECAF

Licença
Projeto de estudo acadêmico. Uso livre para fins educacionais.

use gastore_search::{busca::SistemaBusca, produto::Produto};

#[test]
#[ignore]
pub(crate) fn test_busca_por_nome() {
    let produtos = Produto::carregar_mock();
    let sistema = SistemaBusca::new(&produtos);
    let resultados = sistema.buscar("notebook", None, None);
    assert_eq!(resultados.len(), 1);
    assert_eq!(resultados[0].nome, "Notebook Gamer");
}

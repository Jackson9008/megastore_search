use crate::{produto::Produto, busca::SistemaBusca, grafo::GrafoProdutos};
use eframe::egui::{self, RichText};
use std::collections::HashSet;

pub struct MegaStoreApp {
    nome_busca: String,
    resultados: Vec<Produto>,
    recomendacoes: Vec<String>,
}

impl Default for MegaStoreApp {
    fn default() -> Self {
        Self {
            nome_busca: String::new(),
            resultados: vec![],
            recomendacoes: vec![],
        }
    }
}

impl eframe::App for MegaStoreApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(RichText::new("🔍 MegaStore Search").strong().size(24.0));

            ui.horizontal(|ui| {
                ui.label("Nome do produto:");
                ui.text_edit_singleline(&mut self.nome_busca);
            });

            if ui.button("Buscar").clicked() {
                let produtos = Produto::carregar_mock();
                let sistema = SistemaBusca::new(&produtos);
                let grafo = GrafoProdutos::new(&produtos);

                self.resultados = sistema.buscar(&self.nome_busca, None, None);
                self.recomendacoes = vec![];

                let nomes_encontrados: HashSet<String> = self.resultados.iter().map(|p| p.nome.clone()).collect();

                for nome in &nomes_encontrados {
                    if let Some(similares) = grafo.recomendar(nome) {
                        for s in similares {
                            if !nomes_encontrados.contains(&s) {
                                self.recomendacoes.push(s);
                            }
                        }
                    }
                }
            }

            ui.separator();
            ui.label(RichText::new("📦 Resultados da busca:").strong());

            for p in &self.resultados {
                ui.label(format!("- {} [{} - {}]", p.nome, p.marca, p.categoria));
            }

            if !self.recomendacoes.is_empty() {
                ui.separator();
                ui.label(RichText::new("📦 Recomendações similares:").strong());
                for nome in &self.recomendacoes {
                    ui.label(format!("➡️ {}", nome));
                }
            }
        });
    }
}

mod app;
mod produto;
mod busca;
mod grafo;

use crate::app::MegaStoreApp;



fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "🔍 MegaStore GUI",
        options,
        Box::new(|_cc| Ok(Box::new(MegaStoreApp::default()))),  // ✅ correto

    )
    
}

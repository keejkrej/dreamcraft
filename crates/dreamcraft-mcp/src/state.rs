use dreamcraft_core::EventBus;
use dreamcraft_primitives::tool::{
    CadDrawing, PdfDocument, PhotoCanvas, Presentation, Sequence, VectorDocument, WordDocument,
    Workbook,
};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct DreamSession {
    pub word_doc: Arc<Mutex<Option<WordDocument>>>,
    pub workbook: Arc<Mutex<Option<Workbook>>>,
    pub deck: Arc<Mutex<Option<Presentation>>>,
    pub film_seq: Arc<Mutex<Option<Sequence>>>,
    pub photo_canvas: Arc<Mutex<Option<PhotoCanvas>>>,
    pub vector_doc: Arc<Mutex<Option<VectorDocument>>>,
    pub cad_drawing: Arc<Mutex<Option<CadDrawing>>>,
    pub pdf_doc: Arc<Mutex<Option<PdfDocument>>>,
    pub event_bus: EventBus,
}

impl Default for DreamSession {
    fn default() -> Self {
        Self::new()
    }
}

impl DreamSession {
    pub fn new() -> Self {
        Self {
            word_doc: Arc::new(Mutex::new(Some(WordDocument::new("Untitled Document")))),
            workbook: Arc::new(Mutex::new(Some(Workbook::new("Untitled Workbook")))),
            deck: Arc::new(Mutex::new(Some(Presentation::new("Untitled Presentation")))),
            film_seq: Arc::new(Mutex::new(Some(Sequence::new("Sequence 01")))),
            photo_canvas: Arc::new(Mutex::new(Some(PhotoCanvas::new("Canvas 01", 1920, 1080)))),
            vector_doc: Arc::new(Mutex::new(Some(VectorDocument::new("Artwork 01", 1920.0, 1080.0)))),
            cad_drawing: Arc::new(Mutex::new(Some(CadDrawing::new("Drawing 01")))),
            pdf_doc: Arc::new(Mutex::new(Some(PdfDocument::new("Document 01")))),
            event_bus: EventBus::new(),
        }
    }
}

use dreamcraft_core::EventBus;
use dreamcraft_primitives::tool::{
    CadDrawing, Composition, DesignDocument, LightPhoto, PdfDocument, PhotoCanvas, Presentation,
    Sequence, SoundProject, VectorDocument, WordDocument, Workbook,
};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct DreamSession {
    // Office suite
    pub word_doc: Arc<Mutex<Option<WordDocument>>>,
    pub workbook: Arc<Mutex<Option<Workbook>>>,
    pub deck: Arc<Mutex<Option<Presentation>>>,
    // Video & Motion
    pub film_seq: Arc<Mutex<Option<Sequence>>>,
    pub composition: Arc<Mutex<Option<Composition>>>,
    // Visual & Graphics
    pub photo_canvas: Arc<Mutex<Option<PhotoCanvas>>>,
    pub vector_doc: Arc<Mutex<Option<VectorDocument>>>,
    pub light_photo: Arc<Mutex<Option<LightPhoto>>>,
    // Drafting & Publishing
    pub cad_drawing: Arc<Mutex<Option<CadDrawing>>>,
    pub design_doc: Arc<Mutex<Option<DesignDocument>>>,
    pub pdf_doc: Arc<Mutex<Option<PdfDocument>>>,
    // Audio / Music
    pub sound_project: Arc<Mutex<Option<SoundProject>>>,

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
            composition: Arc::new(Mutex::new(Some(Composition::new("Comp 01", 1920, 1080, 30.0, 300)))),
            photo_canvas: Arc::new(Mutex::new(Some(PhotoCanvas::new("Canvas 01", 1920, 1080)))),
            vector_doc: Arc::new(Mutex::new(Some(VectorDocument::new("Artwork 01", 1920.0, 1080.0)))),
            light_photo: Arc::new(Mutex::new(Some(LightPhoto::new("photo_raw.dng")))),
            cad_drawing: Arc::new(Mutex::new(Some(CadDrawing::new("Drawing 01")))),
            design_doc: Arc::new(Mutex::new(Some(DesignDocument::new("Publication 01")))),
            pdf_doc: Arc::new(Mutex::new(Some(PdfDocument::new("Document 01")))),
            sound_project: Arc::new(Mutex::new(Some(SoundProject::new("Track Session 01")))),
            event_bus: EventBus::new(),
        }
    }
}

import hashlib
from pathlib import Path
import numpy as np
import onnxruntime as ort
from pydantic import BaseModel
from .feature_extractor import FeatureExtractor 

try:
    import lief
except ImportError:
    lief = None

class Prediction(BaseModel):
    sha256: str
    malicious: bool
    confidence: float

class Predictor:
    def __init__(self, model_path: str):
        absolute_model_path = Path(__file__).resolve().parent / model_path
        if not absolute_model_path.exists():
            raise FileNotFoundError(f"Model file not found at: {absolute_model_path}")
            
        self.session = ort.InferenceSession(
            absolute_model_path.as_posix(), providers=["CPUExecutionProvider"]
        )
        self.extractor = FeatureExtractor()
        print(f"ONNX model and EMBER feature extractor loaded successfully.")

    def predict_bytes(self, data: bytes) -> Prediction:
        sha = hashlib.sha256(data).hexdigest()

        # DÜZELTME: Artık özellik çıkarıcının başarısız olma durumunu daha akıllıca yönetiyoruz.
        # FeatureExtractor'ın içindeki try-except bloğu, PE olmayan veya bozuk dosyaları
        # yakalayıp sıfır vektörü döndürecektir.
        features = self.extractor.extract_pe_features(data)
        
        # Eğer özellik çıkarımı başarısız olup sıfır vektörü döndürdüyse,
        # bu, dosyanın analiz edilemediği (muhtemelen PE olmadığı) anlamına gelir.
        if np.sum(features) == 0:
             return Prediction(sha256=sha, malicious=False, confidence=0.01)

        # ONNX modeli ile tahmini çalıştır
        model_inputs = {self.session.get_inputs()[0].name: features}
        model_outputs = self.session.run(None, model_inputs)
        
        probabilities = model_outputs[1][0]
        confidence_score = float(probabilities[1])
        
        return Prediction(
            sha256=sha, 
            malicious=confidence_score > 0.80, # %80 güven eşiğini koruyoruz
            confidence=confidence_score
        )
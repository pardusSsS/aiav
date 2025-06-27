import ember
import numpy as np
import logging

# Loglamayı bu dosya için de etkinleştirelim
logger = logging.getLogger(__name__)

class FeatureExtractor:
    """
    Resmi EMBER kütüphanesini kullanarak bir PE dosyasının byte içeriğinden
    2381 özelliklik bir vektör çıkarır.
    """
    def __init__(self):
        # EMBER'in kendi özellik çıkarıcısını bir üye olarak tutuyoruz.
        self.extractor = ember.features.PEFeatureExtractor(feature_version=2)
        logger.info("Official EMBER Feature Extractor initialized.")

    def extract_pe_features(self, file_data: bytes) -> np.ndarray:
        """
        Dosya içeriğinden EMBER özellik vektörünü çıkarır.
        """
        try:
            # ember.features.PEFeatureExtractor().feature_vector, PE dosyasından
            # doğrudan 2381 özelliklik bir numpy array'i döndürür.
            feature_vector = np.array(self.extractor.feature_vector(file_data), dtype=np.float32)
            
            # Modelin beklediği [1, 2381] şekline getir
            return feature_vector.reshape(1, -1)
            
        except Exception as e:
            # Herhangi bir parsing hatasında (örn: PE olmayan dosya) sıfır vektörü döndür.
            logger.warning(f"Failed to extract EMBER features: {e}. Returning zero vector.")
            return np.zeros((1, self.extractor.dim), dtype=np.float32)
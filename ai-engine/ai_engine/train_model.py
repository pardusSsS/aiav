import numpy as np
from pathlib import Path
from sklearn.ensemble import RandomForestClassifier
from sklearn.metrics import accuracy_score, classification_report
from skl2onnx import convert_sklearn
from skl2onnx.common.data_types import FloatTensorType
import logging

logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')

EMBER_FEATURE_COUNT = 2381
SAMPLES_TO_USE = 400000 

def train_and_save_model():
    script_dir = Path(__file__).resolve().parent
    ai_engine_dir = script_dir.parent
    dataset_path = ai_engine_dir / 'ember_data'
    model_output_dir = script_dir / 'models'
    
    required_files = ['X_train.dat', 'y_train.dat', 'X_test.dat', 'y_test.dat']
    for f in required_files:
        if not (dataset_path / f).exists():
            logging.error(f"Eksik dosya: {(dataset_path / f)}. Lütfen dosyaların doğru yerde olduğundan emin olun.")
            return

    logging.info(f"EMBER veri setinden {SAMPLES_TO_USE} adet örnek yükleniyor...")
    
    X_train = np.memmap(dataset_path / 'X_train.dat', dtype=np.float32, mode='r', shape=(SAMPLES_TO_USE, EMBER_FEATURE_COUNT))
    y_train = np.memmap(dataset_path / 'y_train.dat', dtype=np.float32, mode='r', shape=(SAMPLES_TO_USE,))
    
    logging.info("Etiketlenmemiş veriler filtreleniyor...")
    train_indices = np.where(y_train != -1)[0]
    
    X_train_filtered = X_train[train_indices]
    y_train_filtered = y_train[train_indices].astype(int)
    
    logging.info(f"Filtrelenmiş eğitim verisi boyutu: {len(X_train_filtered)}")
    logging.info(f"Zararlı: {np.sum(y_train_filtered)} | Temiz: {len(y_train_filtered) - np.sum(y_train_filtered)}")

    logging.info("RandomForestClassifier modeli eğitiliyor...")
    model = RandomForestClassifier(n_estimators=100, random_state=42, n_jobs=-1, max_depth=16)
    model.fit(X_train_filtered, y_train_filtered)
    
    # --- Model Değerlendirme (Test verisiyle) ---
    logging.info("Test verisiyle model değerlendiriliyor...")
    X_test = np.memmap(dataset_path / 'X_test.dat', dtype=np.float32, mode='r', shape=(100000, EMBER_FEATURE_COUNT))
    y_test = np.memmap(dataset_path / 'y_test.dat', dtype=np.float32, mode='r', shape=(100000,))
    test_indices = np.where(y_test != -1)[0]
    
    # DÜZELTME: Filtreyi test verisine uygulayarak X_test_filtered'ı oluşturuyoruz.
    X_test_filtered = X_test[test_indices]
    y_test_filtered = y_test[test_indices].astype(int)

    y_pred = model.predict(X_test_filtered)
    logging.info("\n--- Model Değerlendirme Raporu ---")
    logging.info(f"Doğruluk (Accuracy): {accuracy_score(y_test_filtered, y_pred):.4f}")
    print(classification_report(y_test_filtered, y_pred, target_names=['Temiz', 'Zararlı']))
    
    # Modeli ONNX formatına çevirme
    logging.info("Model ONNX formatına dönüştürülüyor...")
    initial_type = [('input', FloatTensorType([None, EMBER_FEATURE_COUNT]))]
    options = {id(model): {'zipmap': False}}
    onnx_model = convert_sklearn(model, initial_types=initial_type, options=options, target_opset=12)
    
    # ONNX modelini kaydetme
    model_output_path = model_output_dir / 'malware_classifier.onnx'
    with open(model_output_path, "wb") as f:
        f.write(onnx_model.SerializeToString())
        
    logging.info(f"Model başarıyla eğitildi ve '{model_output_path}' olarak kaydedildi.")

if __name__ == "__main__":
    train_and_save_model()
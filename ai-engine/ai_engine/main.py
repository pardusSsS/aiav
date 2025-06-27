import uvicorn
import logging
from fastapi import FastAPI, Request, HTTPException
from .predictor import Predictor, Prediction

# Loglama yapılandırması
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(name)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

# FastAPI uygulamasını ve Predictor'ı başlatma
app = FastAPI(
    title="AI-AV Malware Prediction API",
    version="1.0.0"
)
predictor = Predictor(model_path="models/malware_classifier.onnx")
logger.info("Predictor loaded successfully.")

@app.post("/v1/predict", response_model=Prediction)
async def predict(request: Request):
    """
    POST isteği ile ham dosya içeriğini (binary) kabul eder ve bir tahmin (Prediction) döndürür.
    """
    try:
        # İstek gövdesinden dosyanın ham verisini oku
        raw_data = await request.body()
        if not raw_data:
            logger.error("Received empty request body.")
            raise HTTPException(status_code=400, detail="Request body cannot be empty.")
        
        logger.info(f"Received {len(raw_data)} bytes for prediction.")
        
        # Tahmin işlemini predictor'a devret
        prediction_result = predictor.predict_bytes(raw_data)
        
        logger.info(
            f"Prediction for sha256:{prediction_result.sha256} -> "
            f"malicious: {prediction_result.malicious}, confidence: {prediction_result.confidence:.4f}"
        )
        return prediction_result
    
    except Exception as e:
        logger.exception("An error occurred during prediction.")
        raise HTTPException(status_code=500, detail=f"An internal error occurred: {str(e)}")

if __name__ == "__main__":
    # Sunucuyu, Rust istemcisinin erişebileceği şekilde 0.0.0.0 üzerinde başlat
    uvicorn.run(app, host="0.0.0.0", port=8001)
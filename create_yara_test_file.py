import os

# Test edilecek metin
test_string = "Bu dosya AI-Antivirus tarafindan tespit edilmek uzere olusturuldu."

# Dosyanın oluşturulacağı yol
output_path = r'C:\AIAV\TempB\virus.txt'

try:
    # Klasörün var olduğundan emin ol
    os.makedirs(os.path.dirname(output_path), exist_ok=True)

    # Dosyayı, BOM karakteri olmadan, standart UTF-8 formatında yaz.
    with open(output_path, 'w', encoding='utf-8-sig') as f:
        f.write(test_string)

    print(f"Başarılı: '{output_path}' dosyası YARA testi için doğru formatta oluşturuldu.")

except Exception as e:
    print(f"Hata: Dosya oluşturulurken bir sorun oluştu: {e}")
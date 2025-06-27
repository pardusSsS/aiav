import os

# Test dosyasının oluşturulacağı yer ve adı
output_path = r"C:\AIAV\TempA\suspicious_test_file.exe"

# Dosya boyutu (MB cinsinden). Daha büyük dosyalar daha şüpheli görünebilir.
file_size_mb = 5 

try:
    # Tamamen rastgele, yüksek entropili byte'lar oluştur
    random_data = os.urandom(file_size_mb * 1024 * 1024)

    # Dosyayı binary modda yaz
    with open(output_path, 'wb') as f:
        f.write(random_data)

    print(f"Başarılı: '{output_path}' adında {file_size_mb} MB boyutunda yüksek entropili test dosyası oluşturuldu.")

except Exception as e:
    print(f"Hata: {e}")
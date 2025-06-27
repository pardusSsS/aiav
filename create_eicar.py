import os

# Standart ve doğru 68 byte'lık EICAR dizesi
EICAR_STRING = r'X5O!P%@AP[4\PZX54(P^)7CC)7}$EICAR-STANDARD-ANTIVIRUS-TEST-FILE!$H+H*'

# Dosyanın oluşturulacağı tam yol
# Lütfen bu yolun bilgisayarınızda mevcut olduğundan emin olun.
output_directory = r'C:\AIAV\TempA'
output_path = os.path.join(output_directory, 'eicar.com')

try:
    # Klasörün var olduğundan emin ol
    os.makedirs(output_directory, exist_ok=True)
    
    # Dosyayı binary yazma modunda ('wb') açıyoruz.
    # Bu, encoding veya yeni satır karakteri gibi sorunları tamamen ortadan kaldırır.
    with open(output_path, 'wb') as f:
        f.write(EICAR_STRING.encode('ascii'))
        
    print(f"Başarılı: '{output_path}' dosyası doğru içerikle oluşturuldu.")
    print("Şimdi 'hash_checker.py' script'i ile hash'i doğrulayabilirsiniz.")

except Exception as e:
    print(f"Hata: Dosya oluşturulurken bir sorun oluştu: {e}")
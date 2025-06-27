#ifndef FILE_WATCHER_H
#define FILE_WATCHER_H

#include <functional>
#include <string>

// Rust tarafına gönderilecek olay türleri
enum class FileEventType {
    Created,
    Modified,
    Deleted // DÜZELTME: Buradaki '*' karakteri kaldırıldı.
};

// C-stili callback fonksiyon tanımı.
typedef void (*FileEventCallback)(const char* file_path, FileEventType event_type);

#ifdef __cplusplus
extern "C" {
#endif

// İzleyiciyi başlatan ve C-stili callback'i alan C arayüzü fonksiyonu
__declspec(dllexport) void start_file_watcher(const char* path, FileEventCallback callback);

// İzleyiciyi durduran fonksiyon
__declspec(dllexport) void stop_file_watcher();

#ifdef __cplusplus
}
#endif

#endif // FILE_WATCHER_H
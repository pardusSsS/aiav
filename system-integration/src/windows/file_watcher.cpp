#include "file_watcher.h"
#include <windows.h>
#include <thread>
#include <atomic>
#include <vector>
#include <iostream>

static std::atomic<bool> g_stop_flag(false);
static std::thread g_watcher_thread;
static FileEventCallback g_callback = nullptr;

void WatcherLoop(const std::string& path) {
    HANDLE dir_handle = CreateFileA(
        path.c_str(),
        FILE_LIST_DIRECTORY,
        FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
        NULL,
        OPEN_EXISTING,
        FILE_FLAG_BACKUP_SEMANTICS,
        NULL
    );

    if (dir_handle == INVALID_HANDLE_VALUE) {
        std::cerr << "Error: Could not open directory handle. Error code: " << GetLastError() << std::endl;
        return;
    }

    std::vector<BYTE> buffer(65536);

    while (!g_stop_flag) {
        DWORD bytes_returned = 0;
        if (ReadDirectoryChangesW(
            dir_handle,
            buffer.data(),
            // DÜZELTME: C4267 uyarısını gidermek için static_cast eklendi.
            static_cast<DWORD>(buffer.size()),
            TRUE,
            FILE_NOTIFY_CHANGE_FILE_NAME | FILE_NOTIFY_CHANGE_LAST_WRITE | FILE_NOTIFY_CHANGE_DIR_NAME,
            &bytes_returned,
            NULL,
            NULL
        )) {
            if (bytes_returned == 0) continue;

            FILE_NOTIFY_INFORMATION* pNotify = (FILE_NOTIFY_INFORMATION*)buffer.data();
            while (pNotify) {
                wchar_t filename_w[MAX_PATH];
                wcsncpy_s(filename_w, pNotify->FileName, pNotify->FileNameLength / sizeof(wchar_t));
                filename_w[pNotify->FileNameLength / sizeof(wchar_t)] = L'\0';
                
                char filename_c[MAX_PATH];
                size_t converted_chars = 0;
                wcstombs_s(&converted_chars, filename_c, MAX_PATH, filename_w, _TRUNCATE);

                std::string full_path = path + "\\" + filename_c;

                if (g_callback) {
                    FileEventType event_type = FileEventType::Modified;
                    if (pNotify->Action == FILE_ACTION_ADDED || pNotify->Action == FILE_ACTION_RENAMED_NEW_NAME) {
                        event_type = FileEventType::Created;
                    } else if (pNotify->Action == FILE_ACTION_REMOVED || pNotify->Action == FILE_ACTION_RENAMED_OLD_NAME) {
                        event_type = FileEventType::Deleted;
                    }
                    g_callback(full_path.c_str(), event_type);
                }
                
                if (pNotify->NextEntryOffset == 0) break;
                pNotify = (FILE_NOTIFY_INFORMATION*)((BYTE*)pNotify + pNotify->NextEntryOffset);
            }
        } else {
             if (GetLastError() == ERROR_OPERATION_ABORTED) {
                break;
            }
            std::cerr << "ReadDirectoryChangesW failed. Error: " << GetLastError() << std::endl;
            Sleep(5000);
        }
    }
    CloseHandle(dir_handle);
}

void start_file_watcher(const char* path, FileEventCallback callback) {
    if (g_watcher_thread.joinable()) {
        return;
    }
    g_stop_flag = false;
    g_callback = callback;
    g_watcher_thread = std::thread(WatcherLoop, std::string(path));
}

void stop_file_watcher() {
    g_stop_flag = true;
    if (g_watcher_thread.joinable()) {
        g_watcher_thread.join();
    }
    g_callback = nullptr;
}
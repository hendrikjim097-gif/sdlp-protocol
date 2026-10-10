# SDLP Protocol Engine
> **Spatial Distributed Ledger Protocol - Secure P2P Wire Representation & Toroidal Benchmark**

SDLP adalah mesin protokol jaringan peer-to-peer terdistribusi tingkat rendah yang menggunakan topologi ruang metrik toroidal diskret berdimensi 3 (\(\mathcal{T}^3\)) untuk menyelesaikan tabrakan status secara deterministik tanpa interaksi jaringan eksternal.

## ⚡ High-Speed Production Benchmark
Mesin otomasi pembukuan spasial ini telah divalidasi dan diuji menggunakan profil rilis teroptimasi (`--release`) di atas perangkat seluler:
* **Host Pengujian:** Xiaomi Redmi Note 9 (MediaTek Helio G85 ARM CPU, Terisolasi via Termux)
* **Kapasitas Uji:** 100.000 Mutasi Data Spasial Secara Sekaligus
* **Hasil Performa Komputasi:** **~97.293,48 TPS (Transactions Per Second)**

## 🛡️ Fitur Utama Terintegrasi
1. **AEAD AES-256-GCM Wire Protection:** Mengunci 10-byte data awal kerangka jaringan sebagai komponen *Additional Authenticated Data* (AAD) untuk mencegah manipulasi bit pada level kabel jaringan.
2. **16-Bit Delta Compression:** Memeras koordinat spasial 3D dari ukuran standar 12-Byte menjadi hanya 2-Byte murni untuk menghemat bandwidth fisik hingga 83.33%.
3. **Deterministic Spiral Probing:** Mekanisme resolusi tabrakan data berbasis matematika donat terbalik agar setiap node menghasilkan keputusan konvergen yang sama tanpa perlu komunikasi antar-jaringan.

## 📄 Lisensi
Proyek ini dilisensikan di bawah **MIT License**.

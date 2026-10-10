# 📐 Spatial Distributed Ledger Protocol (SDLP)
## Spesifikasi Teknis Formal & Arsitektur Jaringan P2P Spasial

Dokumen ini merinci arsitektur formal SDLP, sebuah protokol pembukuan terdistribusi (*distributed ledger*) yang mengindeks data berdasarkan metrik kedekatan spasial di atas ruang toroidal Gray-Code 3-Dimensi (\(\mathcal{T}^3\)). Protokol ini dirancang untuk mencapai konvergensi status deterministik tanpa overhead interaksi jaringan yang intensif.

---

## 1. Formulasi Matematika Formal (Formal Proofs)

### 1.1 Resolusi Tabrakan Spasial Deterministik (Teorema 1)
Misalkan \(\mathcal{T}^3\) adalah ruang metrik toroidal diskret berdimensi 3 dengan ukuran grid \(M = 2^k\). Fungsi pemetaan koordinat dinamis \(\Psi: \mathbb{A} \times \mathbb{C} \times \mathbb{N} \rightarrow \mathcal{T}^3\) mengambil alamat memori \(\alpha \in \mathbb{A}\), ID Konteks \(\gamma \in \mathbb{C}\), dan urutan langkah resolusi \(s \in \mathbb{N}\) untuk menghasilkan titik koordinat biner Gray-Code akhir \(\mathbf{P}_{\text{final}}\).

Jika dua simpul terdistribusi independen, \(\mathcal{N}_1\) dan \(\mathcal{N}_2\), memiliki keadaan pembukuan lokal (*local state ledger*) yang identik dan menerima argumen input yang sama (α, γ), maka rentetan pencarian koordinat kosong dijamin akan berjalan pada jalur komputasi yang konvergen dan menghasilkan koordinat biner akhir yang identik secara mutlak tanpa interaksi jaringan.

### 1.2 Jarak Hamming Toroidal Konstan (Teorema 2)
Penggunaan Jarak Hamming (\(d_H\)) di atas koordinat Gray-Code pada ruang toroidal \(\mathcal{T}^3\) menjamin eliminasi diskontinuitas spasial. Jika dua alamat memori memiliki kedekatan semantik linear Δ n = 1, maka jarak bit mereka selalu bernilai konstan \(d_H = 1\):
\[d_H(G(n), G(n+1)) = \text{count\_ones}(G(n) \oplus G(n+1)) \equiv 1\]

---

## 2. Format Struktur Data Biner (Wire Protocol Packet Layout)

Setiap paket data pada lapisan transportasi wajib menyelaraskan struktur data kaku berorientasi byte (*byte-aligned packed structure*) untuk zero-copy deserialization:

| Komponen Layout | Ukuran (Byte) | Tipe Data | Deskripsi |
| :--- | :--- | :--- | :--- |
| **Magic Bytes** | 4 | `uint8_t` | Penanda protokol tetap: `0x53 0x44 0x4C 0x50` ("SDLP") |
| **Protocol Version** | 1 | `uint8_t` | Versi protokol saat ini (`0x01`) |
| **Packet Type** | 1 | `uint8_t` | `0x02`: Data Insert (Penyisipan Data) |
| **Payload Length** | 4 | `uint32_t` | Ukuran data muatan setelah header (Big-Endian) |

---

## 3. Skema Kompresi Bitwise Paket (Delta Encoding)
Untuk menghemat bandwidth fisik jaringan hingga **83.33%**, rangkaian koordinat berdekatan dikompresi ke dalam struktur dinamis berukuran **2 Byte (16-bit)** pasca-jangkar basis:

 **Sign Bits (S):** 3-bit pertama menyatakan arah pergeseran koordinat (X, Y, Z). `0` = Positif, `1` = Negatif.
* **Delta X / Y:** Masing-masing 4-bit menyatakan magnitudo pergeseran (Maksimum ±15 langkah).
* **Delta Z:** 5-bit terakhir menyatakan magnitudo pergeseran (Maksimum ±31 langkah).

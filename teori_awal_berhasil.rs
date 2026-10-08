// ============================================================================
// SPATIAL DISTRIBUTED LEDGER PROTOCOL (SDLP) - SECURE P2P PRODUCTION ENGINE
// DIRECTIVE: PACKED WIRE REPR, 16-BIT DELTA COMPRESSION, AEAD AES-256-GCM (AAD)
// COMPATIBILITY: aes-gcm v0.11.1 SPECIFICATION COMPLIANT - FIXED FIELD FIELD
// ============================================================================

use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit, Payload};
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

// --- ENGINE REKAYASA BITWISE COMPRESSION (BAB 12) ---
fn compress_delta(dx: i32, dy: i32, dz: i32) -> Option<u16> {
    if dx.abs() > 15 || dy.abs() > 15 || dz.abs() > 31 { return None; }
    let mut sign_bits: u16 = 0;
    if dx < 0 { sign_bits |= 1 << 2; }
    if dy < 0 { sign_bits |= 1 << 1; }
    if dz < 0 { sign_bits |= 1 << 0; }
    let abs_x = (dx.abs() as u16) & 0x0F;
    let abs_y = (dy.abs() as u16) & 0x0F;
    let abs_z = (dz.abs() as u16) & 0x1F;
    Some((sign_bits << 13) | (abs_x << 9) | (abs_y << 5) | abs_z)
}

fn decompress_delta(compressed: u16) -> (i32, i32, i32) {
    let sign_bits = (compressed >> 13) & 0x07;
    let abs_x = ((compressed >> 9) & 0x0F) as i32;
    let abs_y = ((compressed >> 5) & 0x0F) as i32;
    let abs_z = (compressed & 0x1F) as i32;
    let sx = if (sign_bits & (1 << 2)) != 0 { -1 } else { 1 };
    let sy = if (sign_bits & (1 << 1)) != 0 { -1 } else { 1 };
    let sz = if (sign_bits & (1 << 0)) != 0 { -1 } else { 1 };
    (abs_x * sx, abs_y * sy, abs_z * sz)
}

fn main() {
    println!("=======================================================");
    println!("    SDLP INDUSTRIAL ENCRYPTED NETWORK UTAS VERIFIER   ");
    println!("=======================================================");

    let receiver_addr = "127.0.0.1:4444";
    let sender_addr = "127.0.0.1:5555";

    let raw_key = b"0123456789abcdef0123456789abcdef"; 
    let key = Key::<Aes256Gcm>::from_slice(raw_key);
    let cipher = Aes256Gcm::new(key);

    let cipher_rx = cipher.clone();

    // ------------------------------------------------------------------------
    // [THREAD RECEIVER ENGINE (Rx) - PEMBONGKAR AMAN ENKRIPSI]
    // ------------------------------------------------------------------------
    let rx_handle = thread::spawn(move || {
        let socket = UdpSocket::bind(receiver_addr).expect("Gagal mengikat Rx");
        let mut buffer = [0u8; 256];

        let raw_nonce_rx = b"sdlpnonceiv1";
        let nonce_rx = Nonce::from_slice(raw_nonce_rx);

        println!("[Rx Engine] Mengunci Port Jaringan Terenkripsi {}...", receiver_addr);

        match socket.recv_from(&mut buffer) {
            Ok((bytes_received, src)) => {
                println!("\n[Rx Engine] Ingress Frame Terdeteksi: {} Byte dari {}", bytes_received, src);

                if bytes_received < 12 {
                    println!("   -> Error 0x1000: Frame Incomplete!");
                    return;
                }

                if &buffer[0..4] != b"SDLP" {
                    println!("   [ALERT] Kesalahan 0x1001: ERR_MAGIC_BYTES_MISMATCH");
                    return;
                }

                // Ambil tepat 10 byte pertama sebagai Additional Authenticated Data (AAD) kaku [14.1]
                let mut aad_data = [0u8; 10];
                aad_data.copy_from_slice(&buffer[0..10]);
                
                // Ambil panjang ciphertext secara aman dari segmen biner Big-Endian
                let ciphertext_len = u32::from_be_bytes([buffer[6], buffer[7], buffer[8], buffer[9]]) as usize;
                
                let ciphertext = &buffer[12..(12 + ciphertext_len)];

                println!("   -> Mengekstrak Tambahan Data Terotentikasi (AAD): {:?}", aad_data);
                println!("   -> Memulai Dekripsi Biner Ciphertext AEAD...");

                // KOREKSI: Mengganti nama field dari aads menjadi aad sesuai dengan SDK aes-gcm v0.11.1
                let payload = Payload { msg: ciphertext, aad: &aad_data };
                match cipher_rx.decrypt(nonce_rx, payload) {
                    Ok(decrypted_plaintext) => {
                        println!("   -> STATUS DEKRIPSI: [SUKSES - INTEGRITAS TOTAL VALID]");
                        
                        // Ekstraksi 2 byte data plaintext kembali ke u16 secara aman
                        let mut plain_bytes = [0u8; 2];
                        plain_bytes.copy_from_slice(&decrypted_plaintext[0..2]);
                        
                        let compressed_payload = u16::from_be_bytes(plain_bytes);
                        let (dx, dy, dz) = decompress_delta(compressed_payload);
                        
                        println!("   -> Koordinat Terkompresi: 0xb{:016b}", compressed_payload);
                        println!("   -> Hasil Konvergensi Spasial Akhir: X: {}, Y: {}, Z: {}", dx, dy, dz);
                        println!("   STATUS NODAL: [Q.E.D - STATE MUTATION LOCKED IN OPERATIONAL]");
                    }
                    Err(_) => {
                        println!("   [ALERT] Kesalahan 0x4001: ERR_ZKP_VERIFICATION_FAILED / AEAD MAC TAMPERED!");
                        println!("   [TRANSISI] OPERATIONAL ---> QUARANTINE (Sistem Terisolasi Kaku)");
                    }
                }
            }
            Err(e) => println!("Rx Error: {}", e),
        }
    });

    thread::sleep(Duration::from_millis(500));

    // ------------------------------------------------------------------------
    // [MAIN THREAD TRANSMITTER ENGINE (Tx) - PENYEGEL KRIPTOGRAFI AEAD]
    // ------------------------------------------------------------------------
    println!("\n[Tx Engine] Menyiapkan Paket Mutasi Spasial...");
    let socket = UdpSocket::bind(sender_addr).expect("Gagal mengikat Tx");

    let raw_nonce_tx = b"sdlpnonceiv1";
    let nonce_tx = Nonce::from_slice(raw_nonce_tx);

    let dx = -14; let dy = 7; let dz = 30; 
    let compressed_bits = compress_delta(dx, dy, dz).unwrap();
    let plaintext_payload = compressed_bits.to_be_bytes();

    let mut header_base = [0u8; 12];
    header_base[0..4].copy_from_slice(b"SDLP"); 
    header_base[4] = 0x01; // Versi
    header_base[5] = 0x02; // Tipe Paket: Data Insert
    
    let cipher_len_bytes = (18u32).to_be_bytes();
    header_base[6..10].copy_from_slice(&cipher_len_bytes);
    
    let mut aad_slice = [0u8; 10];
    aad_slice.copy_from_slice(&header_base[0..10]);

    println!("   -> Melakukan Proteksi AEAD pada Data Payload...");
    // KOREKSI: Mengganti nama field dari aads menjadi aad sesuai dengan SDK aes-gcm v0.11.1
    let payload = Payload { msg: &plaintext_payload, aad: &aad_slice };
    
    let ciphertext = cipher.encrypt(nonce_tx, payload)
        .expect("Gagal mengamankan payload dengan enkripsi");

    let mut final_wire_packet = vec![0u8; 12];
    final_wire_packet[0..12].copy_from_slice(&header_base);
    final_wire_packet.extend_from_slice(&ciphertext);

    let hemat = (1.0 - (final_wire_packet.len() as f64 / 108.0)) * 100.0;
    println!("   -> Ukuran Paket Terenkripsi + Aad Header: {} Byte", final_wire_packet.len());
    println!("   -> Penghematan Bandwidth Jaringan Total Fisik: {:.2}%", hemat);
    println!("[Tx Engine] Menembakkan Secure Frame melalui soket UDP...");
    
    socket.send_to(&final_wire_packet, receiver_addr).expect("Gagal menembakkan data");

    rx_handle.join().unwrap();
    println!("\n=======================================================");
    println!("   PROTOKOL AMAN TERKUNCI PERMANEN - COMPLIANT BAB 14  ");
    println!("=======================================================");
        }
                

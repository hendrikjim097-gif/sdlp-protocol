// ============================================================================
// SPATIAL DISTRIBUTED LEDGER PROTOCOL (SDLP) - INTEGRATED PRODUCTION ENGINE
// DIRECTIVE: SECURE WIRE REPR, 16-BIT DELTA COMPRESSION, HIGH-SPEED BENCHMARK
// ============================================================================

use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit, Payload};
use std::collections::HashMap;
use std::net::UdpSocket;
use std::thread;
use std::time::{Duration, Instant};

// --- ENGINE REKAYASA BITWISE COMPRESSION & STORAGE SPASIAL ---
struct SdlpLedgerEngine {
    hot_state_space: HashMap<[u32; 3], String>,
    transit_log_space: Vec<([u32; 3], u64)>,
}

impl SdlpLedgerEngine {
    fn baru() -> Self {
        SdlpLedgerEngine {
            hot_state_space: HashMap::new(),
            transit_log_space: Vec::new(),
        }
    }

    fn cek_terisi(&self, coord: [u32; 3]) -> bool {
        self.hot_state_space.contains_key(&coord)
    }

    fn catat_transaksi(&mut self, coord: [u32; 3], alamat: u64) {
        self.hot_state_space.insert(coord, format!("HP_ID_{}", alamat));
        self.transit_log_space.push((coord, alamat));
    }

    fn bersihkan_memori_pruning(&mut self) {
        self.transit_log_space.clear();
        self.transit_log_space.shrink_to_fit();
    }
}

// Rumus Kecepatan Tinggi Spiral Gray-Code Amendemen 7.1
fn cari_koordinat_spiral(alamat_memori: u64, context_id: u64, grid_m: u32, engine: &SdlpLedgerEngine) -> [u32; 3] {
    let v_alpha = alamat_memori.wrapping_mul(48271);
    let v_gamma = context_id.wrapping_mul(16807);
    
    let x_0 = ((v_alpha as u32) % grid_m) >> 1;
    let y_0 = ((v_alpha as u32) % grid_m) >> 1;
    let z_0 = ((v_alpha as u32) % grid_m) >> 1;
    
    let delta_x_base = ((v_gamma as u32) % 7) + 1;
    let delta_y_base = ((v_gamma as u32) % 5) + 1;
    let delta_z_base = ((v_gamma as u32) % 3) + 1;
    
    let mut s: u32 = 0;
    loop {
        let x_s = (x_0 + (delta_x_base * s)) % grid_m;
        let y_s = (y_0 + (delta_y_base * s)) % grid_m;
        let z_s = (z_0 + (delta_z_base * s)) % grid_m;
        
        let g_x = x_s ^ (x_s >> 1);
        let g_y = y_s ^ (y_s >> 1);
        let g_z = z_s ^ (z_s >> 1);
        
        let kandidat_koordinat = [g_x, g_y, g_z];
        if !engine.cek_terisi(kandidat_koordinat) || s > 100 {
            return kandidat_koordinat;
        }
        s += 1;
    }
}

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
    println!("    SDLP INDUSTRIAL ENCRYPTED JARINGAN P2P VERIFIER   ");
    println!("=======================================================");

    let receiver_addr = "127.0.0.1:4444";
    let sender_addr = "127.0.0.1:5555";

    let raw_key = b"0123456789abcdef0123456789abcdef"; 
    let key = Key::<Aes256Gcm>::from_slice(raw_key);
    let cipher = Aes256Gcm::new(key);
    let cipher_rx = cipher.clone();

    // 1. TIMBANGAN TRANSMISI P2P WIRE ENCRYPTION VIA UDP TEST
    let rx_handle = thread::spawn(move || {
        let socket = UdpSocket::bind(receiver_addr).expect("Gagal mengikat Rx");
        let mut buffer = [0u8; 256];
        let raw_nonce_rx = b"sdlpnonceiv1";
        let nonce_rx = Nonce::from_slice(raw_nonce_rx);

        if let Ok((bytes_received, _)) = socket.recv_from(&mut buffer) {
            if bytes_received >= 12 && &buffer[0..4] == b"SDLP" {
                let mut aad_data = [0u8; 10];
                aad_data.copy_from_slice(&buffer[0..10]);
                let ciphertext_len = u32::from_be_bytes([buffer[6], buffer[7], buffer[8], buffer[9]]) as usize;
                let ciphertext = &buffer[12..(12 + ciphertext_len)];
                let payload = Payload { msg: ciphertext, aad: &aad_data };
                
                if let Ok(decrypted_plaintext) = cipher_rx.decrypt(nonce_rx, payload) {
                    let mut plain_bytes = [0u8; 2];
                    plain_bytes.copy_from_slice(&decrypted_plaintext[0..2]);
                    let compressed_payload = u16::from_be_bytes(plain_bytes);
                    let (dx, dy, dz) = decompress_delta(compressed_payload);
                    println!("[Rx Node] Mutasi Diterima Lolos Verifikasi AEAD -> X: {}, Y: {}, Z: {}", dx, dy, dz);
                }
            }
        }
    });

    thread::sleep(Duration::from_millis(100));

    // Eksekusi Transmitter Wire Frame
    let socket = UdpSocket::bind(sender_addr).expect("Gagal mengikat Tx");
    let raw_nonce_tx = b"sdlpnonceiv1";
    let nonce_tx = Nonce::from_slice(raw_nonce_tx);
    let compressed_bits = compress_delta(-14, 7, 30).unwrap();
    let plaintext_payload = compressed_bits.to_be_bytes();
    
    let mut header_base = [0u8; 12];
    header_base[0..4].copy_from_slice(b"SDLP"); 
    header_base[4] = 0x01; header_base[5] = 0x02;
    header_base[6..10].copy_from_slice(&(18u32).to_be_bytes());

    let payload = Payload { msg: &plaintext_payload, aad: &header_base[0..10] };
    let ciphertext = cipher.encrypt(nonce_tx, payload).unwrap();
    let mut final_wire_packet = vec![0u8; 12];
    final_wire_packet[0..12].copy_from_slice(&header_base);
    final_wire_packet.extend_from_slice(&ciphertext);
    
    socket.send_to(&final_wire_packet, receiver_addr).expect("Gagal kirim");
    rx_handle.join().unwrap();

    // 2. SIMULASI AUTOMATION BENCHMARK 100.000 MUTASI DATA
    println!("\n[INFO] Meluncurkan High-Speed Automation Benchmark...");
    let mut ledger = SdlpLedgerEngine::baru();
    let total_transaksi = 100_000;
    let waktu_mulai = Instant::now();
    
    for i in 0..total_transaksi {
        let alamat_dummy = (i as u64).wrapping_hash();
        let koordinat = cari_koordinat_spiral(alamat_dummy, 12345, 32, &ledger);
        ledger.catat_transaksi(koordinat, alamat_dummy);
        if i % 5_000 == 0 {
            ledger.bersihkan_memori_pruning();
        }
    }
    
    let waktu_total = waktu_mulai.elapsed();
    let total_detik = waktu_total.as_secs_f64();
    println!("   -> Total Data Diproses : {} Mutasi", total_transaksi);
    println!("   -> Kecepatan Mutasi    : {:.2} TPS", (total_transaksi as f64) / total_detik);
    println!("=======================================================");
}

// Perbaikan Trait agar Kompatibel Sempurna dengan Kompilasi Karakter Biner
trait SimpleHash { fn wrapping_hash(self) -> u64; }
impl SimpleHash for u64 {
    fn wrapping_hash(self) -> u64 {
        self.wrapping_mul(6364136223846793005).wrapping_add(1)
    }
}

// ============================================================================
// SPATIAL DISTRIBUTED LEDGER PROTOCOL (SDLP) - BENCHMARK TPS ENGINE AUTOMATION
// DIRECTIVE: HIGH-SPEED TRAVERSAL, SPIRAL PROBING VALIDATION, TIME-ELAPSED
// ============================================================================

use std::collections::HashMap;
use std::time::Instant;

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

fn main() {
    println!("=======================================================");
    println!("    SDLP HIGH-SPEED AUTOMATION BENCHMARK CORE ENGINE   ");
    println!("=======================================================");
    
    let mut ledger = SdlpLedgerEngine::baru();
    let ukuran_grid = 32;
    
    // JUMLAH DATA UJI: Kita hantam langsung dengan 100.000 transaksi!
    let total_transaksi = 100_000;
    
    println!("[INFO] Menyiapkan simulasi {} mutasi data...", total_transaksi);
    println!("[INFO] Memulai stopwatch internal CPU HP...");
    
    // --- STOPWATCH MULAI MENGHITUNG ---
    let waktu_mulai = Instant::now();
    
    for i in 0..total_transaksi {
        // Alamat sengaja diacak dinamis agar memicu tabrakan spasial secara organik
        let alamat_dummy = (i as u64).wrapping_hash();
        
        // 1. Eksekusi Pencarian Koordinat Spiral
        let koordinat = cari_koordinat_spiral(alamat_dummy, 12345, ukuran_grid, &ledger);
        
        // 2. Tulis ke Memori
        ledger.catat_transaksi(koordinat, alamat_dummy);
        
        // 3. Simulasikan Siklus Snapshot & Pruning setiap 5.000 transaksi
        if i % 5_000 == 0 {
            ledger.bersihkan_memori_pruning();
        }
    }
    
    // Hancurkan sisa memori di akhir
    ledger.bersihkan_memori_pruning();
    
    // --- STOPWATCH BERHENTI ---
    let waktu_total = waktu_mulai.elapsed();
    
    // HITUNG MATEMATIKA TPS
    let total_detik = waktu_total.as_secs_f64();
    let tps_riil = (total_transaksi as f64) / total_detik;
    
    println!("\n=======================================================");
    println!("                HASIL PERFORMA HARDWARE                ");
    println!("=======================================================");
    println!("   -> Total Transaksi Diproses : {} Mutasi", total_transaksi);
    println!("   -> Waktu Eksekusi Murni CPU : {:.4} Detik", total_detik);
    println!("   -> KECAPATAN PEMBUKUAN RIIL : {:.2} TPS", tps_riil);
    println!("=======================================================");
}

// Fungsi pembantu untuk mengacak angka secara cepat tanpa membebani sistem
trait SimpleHash { fn wrapping_hash(self) -> u64; }
impl SimpleHash for u64 {
    fn wrapping_hash(self) -> u64 {
        self.wrapping_mul(6364136223846793005).wrapping_add(1)
    }
}


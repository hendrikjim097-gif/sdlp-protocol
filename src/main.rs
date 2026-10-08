use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct SdlpHeader {
    pub magic_bytes: [u8; 4], 
    pub version: u8,          
    pub packet_type: u8,      
    pub payload_length: u32,  
    pub header_checksum: u16, 
}

fn compress_delta(dx: i32, dy: i32, dz: i32) -> Option<u16> {
    if dx.abs() > 15 || dy.abs() > 15 || dz.abs() > 31 {
        return None; 
    }
    let mut sign_bits: u16 = 0;
    if dx < 0 { sign_bits |= 1 << 2; }
    if dy < 0 { sign_bits |= 1 << 1; }
    if dz < 0 { sign_bits |= 1 << 0; }
    let abs_x = (dx.abs() as u16) & 0x0F;
    let abs_y = (dy.abs() as u16) & 0x0F;
    let abs_z = (dz.abs() as u16) & 0x1F;
    let compressed = (sign_bits << 13) | (abs_x << 9) | (abs_y << 5) | abs_z;
    Some(compressed)
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
    println!("      SDLP P2P NETWORK SIMULATOR REAL UDP (ARM/HP)     ");
    println!("=======================================================");
    let receiver_addr = "127.0.0.1:4444";
    let sender_addr = "127.0.0.1:5555";

    let rx_handle = thread::spawn(move || {
        let socket = UdpSocket::bind(receiver_addr).expect("Gagal mengikat Rx Socket");
        let mut buffer = [0u8; 64];
        println!("[Rx Engine] Mendengarkan paket masuk pada port {}...", receiver_addr);
        match socket.recv_from(&mut buffer) {
            Ok((bytes_received, src)) => {
                println!("\n[Rx Engine] Menerima {} byte dari Jaringan UDP ({})", bytes_received, src);
                if bytes_received < 14 { return; }
                let header_ptr = buffer.as_ptr() as *const SdlpHeader;
                let header = unsafe { *header_ptr };
                if &header.magic_bytes != b"SDLP" { return; }
                let compressed_payload = u16::from_be_bytes([buffer[12], buffer[13]]);
                let (dx, dy, dz) = decompress_delta(compressed_payload);
                println!("   -> Hasil Ekstraksi Delta Koordinat: Delta X: {}, Delta Y: {}, Delta Z: {}", dx, dy, dz);
                println!("   STATUS RECEIVER: [MUTLAK KONVERGEN - STATE MACHINE LOCKED]");
            }
            Err(_) => {}
        }
    });

    thread::sleep(Duration::from_millis(500));
    let socket = UdpSocket::bind(sender_addr).expect("Gagal mengikat Tx Socket");
    let delta_x = 5; let delta_y = -12; let delta_z = 25;
    let compressed_bits = compress_delta(delta_x, delta_y, delta_z).unwrap();
    let mut wire_packet = vec![0u8; 14];
    wire_packet[0..4].copy_from_slice(b"SDLP");
    wire_packet[4] = 0x01; wire_packet[5] = 0x02;
    wire_packet[6..10].copy_from_slice(&2u32.to_be_bytes());
    let payload_bytes = compressed_bits.to_be_bytes();
    wire_packet[12] = payload_bytes[0]; wire_packet[13] = payload_bytes[1];

    socket.send_to(&wire_packet, receiver_addr).expect("Gagal mengirim");
    rx_handle.join().unwrap();
                      }
      

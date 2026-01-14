use std::io::{BufRead, Write};

pub struct JavaProc {
    stdin: std::io::BufWriter<std::process::ChildStdin>,
    stdout: std::io::BufReader<std::process::ChildStdout>,
}

impl JavaProc {
    pub fn spawn() -> Self {
        let mut child = std::process::Command::new("java")
            .args(&["-cp", "src", "SlimeServer"])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect("No se pudo ejecutar Java");

        let stdin = std::io::BufWriter::new(child.stdin.take().unwrap());
        let stdout = std::io::BufReader::new(child.stdout.take().unwrap());

        JavaProc { stdin, stdout }
    }

    pub fn query(&mut self, seed: i64, chunk_x: i32, chunk_z: i32) -> bool {
        writeln!(self.stdin, "{} {} {}", seed, chunk_x, chunk_z).unwrap();
        self.stdin.flush().unwrap();

        let mut line = String::new();
        self.stdout.read_line(&mut line).unwrap();
        match line.trim() {
            "1" => true,
            "0" => false,
            other => panic!("Respuesta inesperada de Java: {}", other),
        }
    }
}

fn block_to_chunk(block: i32) -> i32 {
    if block >= 0 {
        block / 16
    } else {
        (block - 15) / 16
    }
}

/// Verifica si el chunk intersecta realmente la esfera de radio 128 bloques.
fn chunk_in_spawn_range(
    chunk_x: i32,
    chunk_z: i32,
    player_x: f64,
    player_z: f64,
    radius: f64,
) -> bool {
    let chunk_min_x = (chunk_x * 16) as f64;
    let chunk_max_x = chunk_min_x + 16.0;
    let chunk_min_z = (chunk_z * 16) as f64;
    let chunk_max_z = chunk_min_z + 16.0;

    // Distancia desde el jugador al centro del chunk más cercano
    let closest_x = player_x.clamp(chunk_min_x, chunk_max_x);
    let closest_z = player_z.clamp(chunk_min_z, chunk_max_z);

    let dx = player_x - closest_x;
    let dz = player_z - closest_z;
    let distance_sq = dx * dx + dz * dz;

    // Solo contamos chunks que estén DENTRO del círculo (no solo tangentes)
    distance_sq < radius * radius
}

/// Cuenta los slime chunks dentro de la esfera de 128 bloques,
/// suponiendo que el jugador está en el bloque (8,7) del chunk actual.
pub fn count_slime_chunks_in_spawn_area(java: &mut JavaProc, seed: i64, chunk_x: i32, chunk_z: i32) -> i32 {
    // Suponemos que el jugador está ligeramente desplazado en el centro superior derecho del chunk
    let player_x = (chunk_x * 16 + 8) as f64;
    let player_z = (chunk_z * 16 + 7) as f64;

    let radius = 128.0;
    let radius_chunks = (radius / 16.0_f64).ceil() as i32;

    let mut count = 0;
    for dx in -radius_chunks..=radius_chunks {
        for dz in -radius_chunks..=radius_chunks {
            let cx = chunk_x + dx;
            let cz = chunk_z + dz;

            if chunk_in_spawn_range(cx, cz, player_x, player_z, radius) {
                if java.query(seed, cx, cz) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn main() {
    let mut java = JavaProc::spawn();
    let seed: i64 = 2521598;
    let chunk_x = 50;
    let chunk_z = -23;

    let slime_count = count_slime_chunks_in_spawn_area(&mut java, seed, chunk_x, chunk_z);
    println!(
        "Desde el chunk ({}, {}), hay {} slime chunks dentro de la esfera de spawn",
        chunk_x, chunk_z, slime_count
    );
}

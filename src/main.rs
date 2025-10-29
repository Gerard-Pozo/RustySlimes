use std::io::{BufRead, BufReader, Write};
use std::process::{ChildStdin, ChildStdout, Command, Stdio};
use std::thread;
use std::time::Duration;

fn block_to_chunk(block: i32) -> i32 {
    if block >= 0 {
        block / 16
    } else {
        (block - 15) / 16
    }
}

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
fn main() -> std::io::Result<()> {
    let mut srv = JavaProc::spawn();

    let seed: i64 = 2521598;
    let block_coords = vec![
        (1496, 69),
        (1512, 20),
        (1459, -57),
        (887, -470),
        (806, -362),
        (789, -364),
    ];

    for (bx, bz) in block_coords {
        let cx = block_to_chunk(bx);
        let cz = block_to_chunk(bz);

        // llama a query directamente
        let slime = srv.query(seed, cx, cz);

        if slime {
            println!("Blocks ({},{}) -> Chunk ({},{}) : SLIME", bx, bz, cx, cz);
        } else {
            println!("Blocks ({},{}) -> Chunk ({},{}) : not slime", bx, bz, cx, cz);
        }
    }

    Ok(())
}
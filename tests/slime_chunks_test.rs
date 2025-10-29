use std::sync::Mutex;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use lazy_static::lazy_static;

// Función de bloque a chunk
fn block_to_chunk(block: i32) -> i32 {
    if block >= 0 { block / 16 } else { (block - 15) / 16 }
}

// Servidor Java persistente
pub struct JavaProc {
    stdin: std::io::BufWriter<std::process::ChildStdin>,
    stdout: BufReader<std::process::ChildStdout>,
}

impl JavaProc {
    pub fn spawn() -> Self {
        let mut child = Command::new("java")
            .args(&["-cp", "src", "SlimeServer"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("No se pudo ejecutar Java");

        let stdin = std::io::BufWriter::new(child.stdin.take().unwrap());
        let stdout = BufReader::new(child.stdout.take().unwrap());

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

    pub fn quit(&mut self) {
        writeln!(self.stdin, "quit").unwrap();
        self.stdin.flush().unwrap();
    }
}

// Un solo servidor compartido para todos los tests
lazy_static! {
    static ref JAVA_PROC: Mutex<JavaProc> = Mutex::new(JavaProc::spawn());
}

#[test]
fn slime_chunks_test() {
    let seed = 2521598;

    // Lista de bloques con valor esperado: (x_block, z_block, es_slime)
    let test_cases = vec![
        (1496, 69, true),
        (1512, 20, false),
        (1459, -57, true),
        (887, -470, false),
        (806, -362, true),
        (789, -364, true),
        (568, -248, true),
        (583, -246, true),
        (597, -249, true),
        (12582, -9161, false),
        (15305, -10618, false),
        (34504, -29864, true),
        (30000000, 30000000, true),
    ];

    for (bx, bz, expected) in test_cases {
        let cx = block_to_chunk(bx);
        let cz = block_to_chunk(bz);

        let mut srv = JAVA_PROC.lock().unwrap();
        let result = srv.query(seed, cx, cz);
        assert_eq!(result, expected, "Coordenada ({}, {})Chunk ({},{}) esperado {}, pero Java devolvió {}",bx, bz, cx, cz, expected, result);
    }
}

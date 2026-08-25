// Módulo: ramati_cognitivo::memoria_contextual
use std::fs::{OpenOptions, File};
use std::io::{Write, Read};

// 1. Definimos los estados posibles que Ramati-Q puede recordar
#[derive(Debug, Clone, Copy)]
pub enum EstadoCognitivo {
    Armonia,
    AngustiaLogica,
    Superposicion, // Estado base (no determinista)
}

// 2. Estructura de la Memoria Persistente Local
pub struct MemoriaContextual {
    pub id_sesion: String,
    pub estado_actual: EstadoCognitivo,
    pub nivel_resonancia: f32, // Probabilidad difusa [0.0 a 1.0]
}

impl MemoriaContextual {
    // Inicializa la memoria en estado puro (Fase Cero)
    pub fn origen(id: &str) -> Self {
        MemoriaContextual {
            id_sesion: id.to_string(),
            estado_actual: EstadoCognitivo::Superposicion,
            nivel_resonancia: 0.5, // 50/50 balance inicial
        }
    }

    // Función para persistir el "pensamiento" en el disco físico
    pub fn colapsar_memoria(&self, ruta_archivo: &str) -> std::io::Result<()> {
        let mut archivo = OpenOptions::new()
            .write(true)
            .create(true)
            .open(ruta_archivo)?;
        
        // Aquí convertimos el estado a bytes puros para máxima velocidad
        let flag_estado: u8 = match self.estado_actual {
            EstadoCognitivo::Armonia => 0x00,
            EstadoCognitivo::AngustiaLogica => 0x01,
            EstadoCognitivo::Superposicion => 0xFF,
        };

        // Escribimos el byte de estado y los 4 bytes del float (nivel_resonancia)
        archivo.write_all(&[flag_estado])?;
        archivo.write_all(&self.nivel_resonancia.to_be_bytes())?;
        
        Ok(())
    }

    // --- FUNCIÓN FINAL: Recuperación Dinámica de Memoria ---
    // Lee los 5 bytes de un archivo .qbin y los decodifica a RAM
    pub fn despertar_memoria(ruta_archivo: &str) -> std::io::Result<Self> {
        let mut archivo = File::open(ruta_archivo)?;
        let mut buffer = Vec::new();
        
        // Leemos los 5 bytes crudos del archivo binario
        archivo.read_to_end(&mut buffer)?;
        
        if buffer.len() < 5 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Memoria corrupta o incompleta"));
        }

        println!("[MEMORIA] Descomprimiendo {} bytes de memoria cuántica desde '{}'...", buffer.len(), ruta_archivo);

        // 1. Decodificamos el primer byte (Estado Cognitivo)
        let estado_recuperado = match buffer[0] {
            0x00 => EstadoCognitivo::Armonia,
            0x01 => EstadoCognitivo::AngustiaLogica,
            _ => EstadoCognitivo::Superposicion,
        };

        // 2. Extraemos los siguientes 4 bytes y los convertimos a f32 (Flotante)
        let mut bytes_float = [0u8; 4];
        bytes_float.copy_from_slice(&buffer[1..5]);
        let resonancia_real = f32::from_be_bytes(bytes_float);

        // Retornamos la estructura con los datos matemáticos reales extraídos
        Ok(MemoriaContextual {
            id_sesion: String::from("sesion_recuperada_dinamica"),
            estado_actual: estado_recuperado,
            nivel_resonancia: resonancia_real,
        })
    }
}
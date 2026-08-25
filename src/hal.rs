// =====================================================================
// Módulo de la Capa de Abstracción de Hardware (HAL) - Ramati-Q v2.0 Prep
// =====================================================================
// Este módulo define el `trait Hardware`, un contrato que cualquier
// dispositivo físico o simulado debe implementar para comunicarse con
// el Kernel de Ramati-Q.

use crate::entorno::ValorRamati;

pub trait Hardware {
    // --- Métodos de Interacción con el Kernel ---

    /// Lee un valor de un sensor específico.
    fn capturar(&self, canal: &str) -> ValorRamati;

    /// Envía un comando a un actuador específico.
    fn accionar(&mut self, actuador: &str, valor: f64);

    /// Establece el objetivo para la navegación.
    fn establecer_objetivo(&mut self, x: f64, y: f64);

    /// Realiza un paso en la simulación o ciclo de hardware.
    fn paso_fisico(&mut self, delta_t: f64);

    /// Devuelve un string con el diagnóstico del estado del hardware.
    fn obtener_diagnostico(&self) -> String;

    /// Verifica si el objetivo ha sido alcanzado.
    fn objetivo_alcanzado(&self) -> bool;
}
// lib.rs

// Definición local de la estructura `Context`.
// Esta estructura representa el contexto del llamante y contiene información sobre quién está realizando la acción.
// - `caller`: Identifica al llamante, generalmente representado como una dirección o identificador único.
pub struct Context {
    pub caller: String, // Dirección o identificador del llamante
}

// Implementación de un autorizador basado en whitelist (lista blanca).
// Este autorizador verifica si un llamante está autorizado para realizar ciertas acciones.
pub struct WhitelistAuthorizer {
    allowed_address: String, // Dirección permitida en la lista blanca
}

impl WhitelistAuthorizer {
    // Constructor para crear una nueva instancia de `WhitelistAuthorizer`.
    // - `allowed_address`: La dirección que se considerará autorizada.
    pub fn new(allowed_address: String) -> Self {
        Self { allowed_address }
    }

    // Método para verificar si un llamante está autorizado.
    // - `ctx`: El contexto del llamante, que incluye su dirección.
    // Retorna `true` si el llamante está autorizado (es decir, su dirección coincide con la dirección permitida).
    pub fn is_authorized(&self, ctx: &Context) -> bool {
        ctx.caller == self.allowed_address
    }
}
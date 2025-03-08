// Importamos el módulo `questions`, que contiene las preguntas y su lógica.
mod questions;

use std::collections::{HashMap, HashSet};
use std::io;

// Importamos estructuras y funciones necesarias desde el SDK `jam-types`.
use jam_types::{Authorizer, CodeHash, AuthParam};

// Importamos solo la función `get_questions` del módulo `questions`.
use questions::get_questions;

// Definimos una estructura `Context` para representar el contexto del llamante.
// Contiene el `caller`, que identifica al usuario que realiza la acción.
pub struct Context {
    pub caller: String, // Dirección o identificador del llamante
}

// Implementación del autorizador basado en roles y whitelist.
// Esta estructura gestiona los roles, direcciones autorizadas y el estado del juego.
pub struct RoleBasedWhitelistAuthorizer {
    allowed_addresses: HashSet<String>, // Lista de direcciones autorizadas
    roles: HashMap<String, String>,     // Mapeo de direcciones a roles ("admin", "user", "observer")
    valid_hashes: Vec<CodeHash>,        // Hashes válidos para la autenticación
    authorizer: Authorizer,             // Instancia del autorizador del SDK
    admin_name: String,                 // Nombre del administrador
    participant_name: String,           // Nombre del participante
    observer_name: String,              // Nombre del observador
    operations_limit: u8,               // Límite de operaciones para el participante
    admin_address: String,              // Dirección predefinida del administrador
    participant_address: String,        // Dirección predefinida del participante
    observer_address: String,           // Dirección predefinida del observador
    answers: Vec<bool>,                 // Respuestas del participante durante el juego
}

impl RoleBasedWhitelistAuthorizer {
    // Constructor para inicializar el autorizador con valores predeterminados.
    pub fn new(
        allowed_addresses: HashSet<String>,
        roles: HashMap<String, String>,
        valid_hashes: Vec<CodeHash>,
        authorizer: Authorizer,
    ) -> Self {
        Self {
            allowed_addresses,
            roles,
            valid_hashes,
            authorizer,
            admin_name: String::new(),
            participant_name: String::new(),
            observer_name: String::new(),
            operations_limit: 0,
            admin_address: "0x1234".to_string(),       // Dirección del administrador
            participant_address: "0x5678".to_string(), // Dirección del participante
            observer_address: "0x9ABC".to_string(),    // Dirección del observador
            answers: Vec::new(),                      // Respuestas del participante
        }
    }

    // Verifica si un llamante está autorizado para realizar una acción específica.
    pub fn is_authorized(&mut self, ctx: &Context, required_role: &str) -> bool {
        // Verificar si el caller está en la lista blanca
        if !self.allowed_addresses.contains(&ctx.caller) {
            return false;
        }

        // Verificar si el caller tiene el rol requerido
        if let Some(role) = self.roles.get(&ctx.caller) {
            if role == required_role {
                // Verificar si el code_hash es válido
                if self.valid_hashes.contains(&self.authorizer.code_hash) {
                    // Si el rol es 'admin', almacenar el caller en el param del Authorizer
                    if role == "admin" {
                        let mut admin_param = AuthParam::new();
                        admin_param.extend(ctx.caller.as_bytes()); // Convertir String a &[u8]
                        self.authorizer.param = admin_param;

                        // Mostrar el contenido del AuthParam para depurar
                        println!(
                            "Updated Auth Param: {:?}",
                            self.authorizer.param.0
                        );
                    }
                    return true;
                }
            }
        }

        false
    }

    // Configura el juego solicitando los nombres del administrador, participante y observador.
    pub fn setup_game(&mut self) {
        // Paso 1: Pedir el nombre del administrador
        println!("Indique el nombre del administrador:");
        let mut admin_name = String::new();
        io::stdin().read_line(&mut admin_name).expect("Error al leer el nombre del administrador");
        self.admin_name = admin_name.trim().to_string();

        // Paso 2: Pedir el nombre del participante
        println!("Ahora indique el nombre del participante:");
        let mut participant_name = String::new();
        io::stdin()
            .read_line(&mut participant_name)
            .expect("Error al leer el nombre del participante");
        self.participant_name = participant_name.trim().to_string();

        // Paso 3: Pedir el nombre del observador
        println!("Finalmente, indique el nombre del observador:");
        let mut observer_name = String::new();
        io::stdin()
            .read_line(&mut observer_name)
            .expect("Error al leer el nombre del observador");
        self.observer_name = observer_name.trim().to_string();

        // Generar un Code Hash dinámico basado en los datos ingresados
        let dynamic_code_hash = CodeHash::padded(self.admin_name.as_bytes());
        self.valid_hashes.push(dynamic_code_hash.clone());
        self.authorizer.code_hash = dynamic_code_hash;

        // Asociar roles con direcciones específicas
        self.roles.insert(self.admin_address.clone(), "admin".to_string());
        self.roles.insert(self.participant_address.clone(), "user".to_string());
        self.roles.insert(self.observer_address.clone(), "observer".to_string());

        // Añadir direcciones a la lista blanca
        self.allowed_addresses.insert(self.admin_address.clone());
        self.allowed_addresses.insert(self.participant_address.clone());
        self.allowed_addresses.insert(self.observer_address.clone());

        // Crear un contexto simulado para el administrador
        let admin_ctx = Context {
            caller: self.admin_address.clone(),
        };

        // Autorizar al administrador para actualizar el AuthParam
        self.is_authorized(&admin_ctx, "admin");
    }

    // Muestra el menú de opciones y permite interactuar con el juego.
    pub fn show_menu(&mut self) {
        loop {
            println!("\n--- Menú de opciones ---");
            println!("1. Definir número de operaciones (Administrador)");
            println!("2. Jugar (Participante)");
            println!("3. Ver resultados (Observador/Administrador)");
            println!("4. Mostrar salida final y salir");

            let mut choice = String::new();
            println!("Seleccione una opción:");
            io::stdin().read_line(&mut choice).expect("Error al leer la opción");
            let choice = choice.trim().parse::<u8>().unwrap_or(0);

            match choice {
                1 => self.set_operations_limit(),
                2 => self.play_game(),
                3 => self.view_results(),
                4 => {
                    self.show_final_output();
                    break;
                }
                _ => println!("Opción inválida. Intente nuevamente."),
            }
        }
    }

    // Opción 1: Permite al administrador definir el número de operaciones para el participante.
    fn set_operations_limit(&mut self) {
        println!("Ingrese su nombre para verificar su rol de administrador:");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Error al leer el nombre");
        let name = name.trim();

        if name == self.admin_name {
            println!("Hola {}, indique un número entre 1 y 5 de operaciones que puede hacer el participante:", self.admin_name);
            let mut operations_limit = String::new();
            io::stdin()
                .read_line(&mut operations_limit)
                .expect("Error al leer el número de operaciones");
            self.operations_limit = operations_limit.trim().parse::<u8>().unwrap_or(1).clamp(1, 5);
            println!("Se han definido {} operaciones para el participante.", self.operations_limit);
        } else {
            println!("Nombre incorrecto. Solo el administrador puede realizar esta acción.");
        }
    }

    // Opción 2: Permite al participante jugar respondiendo preguntas.
    fn play_game(&mut self) {
        println!("Ingrese su nombre para verificar su rol de participante:");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Error al leer el nombre");
        let name = name.trim();

        if name == self.participant_name {
            if self.operations_limit == 0 {
                println!("El administrador aún no ha definido el número de operaciones.");
                return;
            }

            // Obtener las preguntas desde el módulo `questions`
            let questions = get_questions();

            println!("Hola {}, comienza el juego. Responde las siguientes preguntas:", self.participant_name);
            for (i, question) in questions.iter().enumerate().take(self.operations_limit as usize) {
                println!("\n{} - {}", i + 1, question.text);

                match question.question_type.as_str() {
                    "yes_no" => {
                        println!("Respuesta (sí/no):");
                        let mut answer = String::new();
                        io::stdin().read_line(&mut answer).expect("Error al leer la respuesta");
                        let answer = answer.trim().to_lowercase();
                        let response = match answer.as_str() {
                            "si" | "sí" => true,
                            "no" => false,
                            _ => {
                                println!("Respuesta inválida. Se considerará 'no'.");
                                false
                            }
                        };
                        self.answers.push(response);
                    }
                    "multiple_choice" => {
                        if let Some(options) = &question.options {
                            println!("Opciones disponibles:");
                            for (idx, option) in options.iter().enumerate() {
                                println!("{}. {}", idx + 1, option);
                            }
                            println!("Seleccione una opción (número):");
                            let mut choice = String::new();
                            io::stdin().read_line(&mut choice).expect("Error al leer la opción");
                            let choice = choice.trim().parse::<usize>().unwrap_or(0);

                            if choice > 0 && choice <= options.len() {
                                println!("Seleccionaste: {}", options[choice - 1]);
                                self.answers.push(true); // Marcar como respondido
                            } else {
                                println!("Opción inválida.");
                                self.answers.push(false); // Marcar como no respondido
                            }
                        }
                    }
                    "numeric" => {
                        println!("Respuesta (número):");
                        let mut answer = String::new();
                        io::stdin().read_line(&mut answer).expect("Error al leer la respuesta");
                        let answer = answer.trim().parse::<i32>().unwrap_or(-1);

                        if let Some(correct_answer) = &question.answer {
                            let correct_answer = correct_answer.parse::<i32>().unwrap_or(-1);
                            if answer == correct_answer {
                                println!("¡Respuesta correcta!");
                                self.answers.push(true);
                            } else {
                                println!("Respuesta incorrecta.");
                                self.answers.push(false);
                            }
                        }
                    }
                    _ => {
                        println!("Tipo de pregunta no soportado.");
                        self.answers.push(false);
                    }
                }
            }
            println!("Has completado todas las operaciones.");
        } else {
            println!("Nombre incorrecto. Solo el participante puede realizar esta acción.");
        }
    }

    // Opción 3: Permite al observador o administrador ver los resultados del participante.
    fn view_results(&mut self) {
        println!("Ingrese su nombre para verificar su rol de observador o administrador:");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Error al leer el nombre");
        let name = name.trim();

        if name == self.observer_name || name == self.admin_name {
            if self.answers.is_empty() {
                println!("Aún no hay resultados disponibles.");
                return;
            }

            let questions = get_questions();

            println!("Resultados del participante ({}):", self.participant_name);
            for (i, (answer, question)) in self.answers.iter().zip(questions.iter()).enumerate() {
                println!(
                    "{} - {}: {}",
                    i + 1,
                    question.text,
                    if *answer { "Correcto" } else { "Incorrecto" }
                );
            }
        } else {
            println!("Nombre incorrecto. Solo el observador o administrador pueden ver los resultados.");
        }
    }

    // Opción 4: Muestra la salida final del programa.
    fn show_final_output(&self) {
        println!(
            "\nHola {}({}), de acuerdo a {}({}), puedes realizar {} operaciones.",
            self.participant_name,
            self.participant_address,
            self.admin_name,
            self.admin_address,
            self.operations_limit
        );
        println!(
            "Hola {}({}), eres un observador y puedes ver las acciones del sistema.",
            self.observer_name,
            self.observer_address
        );
        println!(
            "Final Code Hash: {:?}",
            self.authorizer.code_hash.0
        );
        println!(
            "Final Auth Param: {:?}",
            self.authorizer.param.0
        );
    }
}

fn main() {
    // Crear una instancia de Authorizer con valores iniciales
    let code_hash = CodeHash::padded(b"initial_hash"); // Valor inicial
    let param = AuthParam::new(); // Parámetro vacío inicialmente
    let authorizer = Authorizer { code_hash, param };

    // Crear una lista de direcciones autorizadas (vacía inicialmente)
    let allowed_addresses = HashSet::new();

    // Crear un mapa de roles (vacío inicialmente)
    let roles = HashMap::new();

    // Crear una lista de hashes válidos (vacía inicialmente)
    let valid_hashes = Vec::new();

    // Crear una instancia del autorizador personalizado
    let mut role_based_authorizer = RoleBasedWhitelistAuthorizer::new(
        allowed_addresses,
        roles,
        valid_hashes,
        authorizer,
    );

    // Configurar el juego
    role_based_authorizer.setup_game();

    // Mostrar el menú de opciones
    role_based_authorizer.show_menu();
}
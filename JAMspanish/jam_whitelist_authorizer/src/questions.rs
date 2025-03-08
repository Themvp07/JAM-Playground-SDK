// questions.rs

// Definimos una estructura `Question` para representar las preguntas del juego.
// Esta estructura contiene todos los datos necesarios para manejar diferentes tipos de preguntas.
#[derive(Debug, Clone)] // Derivamos `Debug` para facilitar la depuración y `Clone` para permitir copias.
pub struct Question {
    pub text: String,               // Texto de la pregunta que se mostrará al participante.
    pub question_type: String,      // Tipo de pregunta ("yes_no", "multiple_choice", "numeric").
    pub options: Option<Vec<String>>, // Opciones disponibles para preguntas de múltiple elección.
                                       // Es `None` si la pregunta no es de este tipo.
    pub answer: Option<String>,     // Respuesta válida (opcional). Es `None` si no hay respuesta correcta.
}

// Función para obtener un vector de preguntas predefinidas.
// Estas preguntas se utilizan en el juego y pueden ser modificadas o ampliadas según sea necesario.
pub fn get_questions() -> Vec<Question> {
    vec![
        // Pregunta 1: Una pregunta de tipo "sí/no".
        Question {
            text: "¿Es de día?".to_string(), // Texto de la pregunta.
            question_type: "yes_no".to_string(), // Tipo de pregunta: "sí/no".
            options: None, // No hay opciones porque es una pregunta de tipo "sí/no".
            answer: Some("si".to_string()), // Respuesta válida: "si".
        },
        // Pregunta 2: Una pregunta de tipo "múltiple elección".
        Question {
            text: "¿Cuál es tu color favorito?".to_string(), // Texto de la pregunta.
            question_type: "multiple_choice".to_string(), // Tipo de pregunta: "múltiple elección".
            options: Some(vec![ // Opciones disponibles para esta pregunta.
                "rojo".to_string(),
                "azul".to_string(),
                "verde".to_string(),
            ]),
            answer: None, // No hay respuesta correcta para esta pregunta.
        },
        // Pregunta 3: Una pregunta de tipo "numérica".
        Question {
            text: "¿Cuántos dedos tienes en una mano?".to_string(), // Texto de la pregunta.
            question_type: "numeric".to_string(), // Tipo de pregunta: "numérica".
            options: None, // No hay opciones porque es una pregunta numérica.
            answer: Some("5".to_string()), // Respuesta válida: "5".
        },
    ]
}
// questions.rs

// We define a `Question` struct to represent the game's questions.
// This struct contains all the necessary data to handle different types of questions.
#[derive(Debug, Clone)] // We derive `Debug` to facilitate debugging and `Clone` to allow copying.
pub struct Question {
    pub text: String,               // The question text that will be shown to the participant.
    pub question_type: String,      // The type of question ("yes_no", "multiple_choice", "numeric").
    pub options: Option<Vec<String>>, // Available options for multiple-choice questions.
                                       // It is `None` if the question is not of this type.
    pub answer: Option<String>,     // Valid answer (optional). It is `None` if there is no correct answer.
}

// Function to get a vector of predefined questions.
// These questions are used in the game and can be modified or expanded as needed.
pub fn get_questions() -> Vec<Question> {
    vec![
        // Question 1: A "yes/no" type question.
        Question {
            text: "Is it daytime?".to_string(), // The question text.
            question_type: "yes_no".to_string(), // Question type: "yes/no".
            options: None, // No options because it's a "yes/no" question.
            answer: Some("yes".to_string()), // Valid answer: "yes".
        },
        // Question 2: A "multiple choice" type question.
        Question {
            text: "What is your favorite color?".to_string(), // The question text.
            question_type: "multiple_choice".to_string(), // Question type: "multiple choice".
            options: Some(vec![ // Available options for this question.
                "red".to_string(),
                "blue".to_string(),
                "green".to_string(),
            ]),
            answer: None, // There is no correct answer for this question.
        },
        // Question 3: A "numeric" type question.
        Question {
            text: "How many Dots are in the Polkadot logo?".to_string(), // The question text.
            question_type: "numeric".to_string(), // Question type: "numeric".
            options: None, // No options because it's a numeric question.
            answer: Some("6".to_string()), // Valid answer: "6".
        },
    ]
}
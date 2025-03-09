// Import the `questions` module, which contains the questions and their logic.
mod questions;

use std::collections::{HashMap, HashSet};
use std::io;

// Import necessary structures and functions from the `jam-types` SDK.
use jam_types::{Authorizer, CodeHash, AuthParam};

// Import only the `get_questions` function from the `questions` module.
use questions::get_questions;

// Define a `Context` struct to represent the caller's context.
// It contains the `caller`, which identifies the user performing the action.
pub struct Context {
    pub caller: String, // Caller's address or identifier
}

// Implementation of the role-based and whitelist authorizer.
// This struct manages roles, authorized addresses, and the game state.
pub struct RoleBasedWhitelistAuthorizer {
    allowed_addresses: HashSet<String>, // List of authorized addresses
    roles: HashMap<String, String>,     // Mapping of addresses to roles ("admin", "user", "observer")
    valid_hashes: Vec<CodeHash>,        // Valid hashes for authentication
    authorizer: Authorizer,             // Instance of the SDK's authorizer
    admin_name: String,                 // Admin's name
    participant_name: String,           // Participant's name
    observer_name: String,              // Observer's name
    questions_limit: u8,               // Operation limit for the participant
    admin_address: String,              // Predefined admin address
    participant_address: String,        // Predefined participant address
    observer_address: String,           // Predefined observer address
    answers: Vec<bool>,                 // Participant's answers during the game
}

impl RoleBasedWhitelistAuthorizer {
    // Constructor to initialize the authorizer with default values.
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
            questions_limit: 0,
            admin_address: "0x1234".to_string(),       // Admin address
            participant_address: "0x5678".to_string(), // Participant address
            observer_address: "0x9ABC".to_string(),    // Observer address
            answers: Vec::new(),                      // Participant's answers
        }
    }

    // Check if a caller is authorized to perform a specific action.
    pub fn is_authorized(&mut self, ctx: &Context, required_role: &str) -> bool {
        // Verify if the caller is in the whitelist
        if !self.allowed_addresses.contains(&ctx.caller) {
            return false;
        }

        // Verify if the caller has the required role
        if let Some(role) = self.roles.get(&ctx.caller) {
            if role == required_role {
                // Verify if the code_hash is valid
                if self.valid_hashes.contains(&self.authorizer.code_hash) {
                    // If the role is 'admin', store the caller in the Authorizer's param
                    if role == "admin" {
                        let mut admin_param = AuthParam::new();
                        admin_param.extend(ctx.caller.as_bytes()); // Convert String to &[u8]
                        self.authorizer.param = admin_param;

                        // Display the AuthParam content for debugging
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

    // Set up the game by requesting the names of the admin, participant, and observer.
    pub fn setup_game(&mut self) {
        // Step 1: Ask for the admin's name
        println!("Enter the admin's name:");
        let mut admin_name = String::new();
        io::stdin().read_line(&mut admin_name).expect("Failed to read admin's name");
        self.admin_name = admin_name.trim().to_string();

        // Step 2: Ask for the participant's name
        println!("Now enter the participant's name:");
        let mut participant_name = String::new();
        io::stdin()
            .read_line(&mut participant_name)
            .expect("Failed to read participant's name");
        self.participant_name = participant_name.trim().to_string();

        // Step 3: Ask for the observer's name
        println!("Finally, enter the observer's name:");
        let mut observer_name = String::new();
        io::stdin()
            .read_line(&mut observer_name)
            .expect("Failed to read observer's name");
        self.observer_name = observer_name.trim().to_string();

        // Generate a dynamic Code Hash based on the entered data
        let dynamic_code_hash = CodeHash::padded(self.admin_name.as_bytes());
        self.valid_hashes.push(dynamic_code_hash.clone());
        self.authorizer.code_hash = dynamic_code_hash;

        // Associate roles with specific addresses
        self.roles.insert(self.admin_address.clone(), "admin".to_string());
        self.roles.insert(self.participant_address.clone(), "user".to_string());
        self.roles.insert(self.observer_address.clone(), "observer".to_string());

        // Add addresses to the whitelist
        self.allowed_addresses.insert(self.admin_address.clone());
        self.allowed_addresses.insert(self.participant_address.clone());
        self.allowed_addresses.insert(self.observer_address.clone());

        // Create a simulated context for the admin
        let admin_ctx = Context {
            caller: self.admin_address.clone(),
        };

        // Authorize the admin to update the AuthParam
        self.is_authorized(&admin_ctx, "admin");
    }

    // Display the menu and allow interaction with the game.
    pub fn show_menu(&mut self) {
        loop {
            println!("\n--- Menu Options ---");
            println!("1. Set number of questions (Admin)");
            println!("2. Play (Participant)");
            println!("3. View results (Observer/Admin)");
            println!("4. Show final output and exit");

            let mut choice = String::new();
            println!("Select an option:");
            io::stdin().read_line(&mut choice).expect("Failed to read option");
            let choice = choice.trim().parse::<u8>().unwrap_or(0);

            match choice {
                1 => self.set_questions_limit(),
                2 => self.play_game(),
                3 => self.view_results(),
                4 => {
                    self.show_final_output();
                    break;
                }
                _ => println!("Invalid option. Please try again."),
            }
        }
    }

    // Option 1: Allow the admin to set the number of questions for the participant.
    fn set_questions_limit(&mut self) {
        println!("Enter your name to verify your admin role:");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read name");
        let name = name.trim();

        if name == self.admin_name {
            println!("Hello {}, enter a number between 1 and 3 for the participant's questions:", self.admin_name);
            let mut questions_limit = String::new();
            io::stdin()
                .read_line(&mut questions_limit)
                .expect("Failed to read the number of questions");
            self.questions_limit = questions_limit.trim().parse::<u8>().unwrap_or(1).clamp(1, 3);
            println!("{} questions have been set for the participant.", self.questions_limit);
        } else {
            println!("Incorrect name. Only the admin can perform this action.");
        }
    }

    // Option 2: Allow the participant to play by answering questions.
    fn play_game(&mut self) {
        println!("Enter your name to verify your participant role:");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read name");
        let name = name.trim();

        if name == self.participant_name {
            if self.questions_limit == 0 {
                println!("The admin has not yet set the number of questions.");
                return;
            }

            // Get the questions from the `questions` module
            let questions = get_questions();

            println!("Hello {}, the game begins. Answer the following questions:", self.participant_name);
            for (i, question) in questions.iter().enumerate().take(self.questions_limit as usize) {
                println!("\n{} - {}", i + 1, question.text);

                match question.question_type.as_str() {
                    "yes_no" => {
                        println!("Answer (yes/no):");
                        let mut answer = String::new();
                        io::stdin().read_line(&mut answer).expect("Failed to read answer");
                        let answer = answer.trim().to_lowercase();
                        let response = match answer.as_str() {
                            "yes" | "y" => true,
                            "no" | "n" => false,
                            _ => {
                                println!("Invalid answer. It will be considered 'no'.");
                                false
                            }
                        };
                        self.answers.push(response);
                    }
                    "multiple_choice" => {
                        if let Some(options) = &question.options {
                            println!("Available options:");
                            for (idx, option) in options.iter().enumerate() {
                                println!("{}. {}", idx + 1, option);
                            }
                            println!("Select an option (number):");
                            let mut choice = String::new();
                            io::stdin().read_line(&mut choice).expect("Failed to read choice");
                            let choice = choice.trim().parse::<usize>().unwrap_or(0);

                            if choice > 0 && choice <= options.len() {
                                println!("You selected: {}", options[choice - 1]);
                                self.answers.push(true); // Mark as answered
                            } else {
                                println!("Invalid option.");
                                self.answers.push(false); // Mark as unanswered
                            }
                        }
                    }
                    "numeric" => {
                        println!("Answer (number):");
                        let mut answer = String::new();
                        io::stdin().read_line(&mut answer).expect("Failed to read answer");
                        let answer = answer.trim().parse::<i32>().unwrap_or(-1);

                        if let Some(correct_answer) = &question.answer {
                            let correct_answer = correct_answer.parse::<i32>().unwrap_or(-1);
                            if answer == correct_answer {
                                println!("Correct answer!");
                                self.answers.push(true);
                            } else {
                                println!("Incorrect answer.");
                                self.answers.push(false);
                            }
                        }
                    }
                    _ => {
                        println!("Unsupported question type.");
                        self.answers.push(false);
                    }
                }
            }
            println!("You have completed all questions.");
        } else {
            println!("Incorrect name. Only the participant can perform this action.");
        }
    }

    // Option 3: Allow the observer or admin to view the participant's results.
    fn view_results(&mut self) {
        println!("Enter your name to verify your observer or admin role:");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read name");
        let name = name.trim();

        if name == self.observer_name || name == self.admin_name {
            if self.answers.is_empty() {
                println!("No results available yet.");
                return;
            }

            let questions = get_questions();

            println!("Participant's results ({}):", self.participant_name);
            for (i, (answer, question)) in self.answers.iter().zip(questions.iter()).enumerate() {
                println!(
                    "{} - {}: {}",
                    i + 1,
                    question.text,
                    if *answer { "Correct" } else { "Incorrect" }
                );
            }
        } else {
            println!("Incorrect name. Only the observer or admin can view the results.");
        }
    }

    // Option 4: Display the final output of the program.
    fn show_final_output(&self) {
        println!(
            "\nHello {}({}), according to {}({}), you can perform {} questions.",
            self.participant_name,
            self.participant_address,
            self.admin_name,
            self.admin_address,
            self.questions_limit
        );
        println!(
            "Hello {}({}), you are an observer and can view system actions.",
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
    // Create an instance of Authorizer with initial values
    let code_hash = CodeHash::padded(b"initial_hash"); // Initial value
    let param = AuthParam::new(); // Initially empty parameter
    let authorizer = Authorizer { code_hash, param };

    // Create a list of authorized addresses (initially empty)
    let allowed_addresses = HashSet::new();

    // Create a role map (initially empty)
    let roles = HashMap::new();

    // Create a list of valid hashes (initially empty)
    let valid_hashes = Vec::new();

    // Create an instance of the custom authorizer
    let mut role_based_authorizer = RoleBasedWhitelistAuthorizer::new(
        allowed_addresses,
        roles,
        valid_hashes,
        authorizer,
    );

    // Set up the game
    role_based_authorizer.setup_game();

    // Display the menu options
    role_based_authorizer.show_menu();
}
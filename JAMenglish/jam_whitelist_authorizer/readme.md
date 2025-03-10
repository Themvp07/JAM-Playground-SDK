# **JAM Whitelist Authorizer**

This project implements a **role-based and whitelist authorizer** using the JAM SDK. It allows managing authorized addresses, user roles (admin, participant, observer), and an interactive game where participants answer questions. The results are supervised by observers and administrators.

---

## **Table of Contents**

1. [Project Description](#project-description)
2. [JAM SDK Components](#jam-sdk-components)
3. [System Requirements](#system-requirements)
4. [Installation](#installation)
5. [Running the Project](#running-the-project)
6. [Project Structure](#project-structure)
7. [Customization](#customization)
8. [Contributions](#contributions)

---

## **Project Description**

This program uses the JAM SDK to implement a role-based and whitelist authorization system. It includes the following functionalities:

- **Roles**: Admin, participant, and observer.
- **Whitelist**: Only authorized addresses can perform certain actions.
- **Interactive Game**: The participant answers questions defined by the admin.
- **Results**: The game results are supervised by the observer or admin.

The project is designed to be modular and easy to extend, allowing the addition of new questions, roles, or functionalities.

---

## **JAM SDK Components**

The JAM SDK provides several key tools and structures that are used in this project. Below is a description of how these components are integrated into the current code, along with suggestions for additional functionalities that could be implemented in the future::

### **1. `jam-types` (Implemented)**
- **Description**: This crate contains basic types and structures needed for working with services and authorizers in JAM.
- **Components Used**:
  - **`Authorizer`**: Represents the JAM system's authorizer. It contains two main fields:
    - `code_hash`: A hash representing the program code.
    - `param`: A parameter that can store additional information (e.g., the admin's address).
  - **`CodeHash`**: Represents a 32-byte hash used to validate whether a hash belongs to a list of valid hashes.
  - **`AuthParam`**: A parameter used to store additional data related to authorization.
- **Usage in the Project**:
  - `Authorizer` is used to manage the validation of authorized addresses.
  - A `CodeHash` is dynamically generated based on the admin's name and used to verify the authenticity of operations.
  - `AuthParam` is extended with the admin's address to track who performed certain actions.

### **2. `jam-pvm-common` (Reviewed for Future Implementation)**
- **Description**: Provides common functionalities for working with the PVM (Parity Virtual Machine).
- **Current Usage**:
  - Not explicitly used in the current code.
  - **Future Suggestion**: It could be used to ensure that data generated by the authorizer is compatible with the PVM runtime environment. For example, you could implement specific serialization/deserialization functions to handle data in the format required by the PVM.

### **3. `jam-bootstrap-service` (Reviewed, Used as Reference)**
- **Description**: A basic service useful for creating initial configurations (genesis) in JAM.
- **Usage in the Project**:
  - Cloned as a reference to understand how to configure a basic service in JAM.
  - **Future Suggestion**: It could be used to define an initial configuration that includes predefined addresses for the admin, participant, and observer.

### **4. `jam-null-authorizer` (Used as a Starting Point, but Not Present in the Current Code)**
- **Description**: A basic authorizer that allows testing without authorization restrictions.
- **Current Usage**:
  - Not explicitly used in the current code.
  - **Future Suggestion**: It could be used as a starting point to implement more advanced test cases. For example, it could be integrated to test scenarios where no authorization is required.


### **5. `jam-pvm-build` (Reviewed but Not Publicly Available)**
- **Description**: A CLI tool that allows building PVM code blobs for services or authorizers.
- **Current Usage**:
  - Not explicitly used in the current code.
  - **Future Suggestion**: It could be used to compile the custom authorizer into a deployable PVM blob.

Main JAM SDK resources link: https://hackmd.io/@polkadot/jamsdk

---

## **System Requirements**

To run this project, you need the following:

1. **Operating System**:
   - Linux, macOS, or Windows (with support for command-line tools like Git Bash or PowerShell).

2. **Required Tools**:
   - [Git](https://git-scm.com/)
   - [Rust](https://www.rust-lang.org/) (version `nightly-2024-11-01` or higher)
   - Cargo (Rust's package manager, installed automatically with Rust)

3. **JAM SDK**:
   - Specific SDK dependencies (`jam-types`, `jam-pvm-common`, etc.).

---

## **Installation**

Follow these steps to set up the environment and download the necessary dependencies:

### **1. Install Rust**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. Install the specific toolchain
```
rustup toolchain install nightly-2024-11-01 -c rust-src
rustup default nightly-2024-11-01
```
### 3. Clone the repository
```
git clone https://github.com/Themvp07/JAM-Playground-SDK.git
cd JAMspanish/jam_whitelist_authorizer
```

### 4. Install additional tools
```
cargo install cargo-clone
cargo install jam-pvm-build
```

### 5. Clone necessary crates
```
cargo clone jam-bootstrap-service
cargo clone jam-null-authorizer
```

### 6. Build the project
```
cargo build
```

---

## **Running the Project**

Once the environment is set up, you can run the project with the following command:

```
cargo run
```
### **Program Flow**

1. **Initial Setup**:
   - Enter the names of the admin, participant, and observer.
   - Define the number of operations the participant can perform.

2. **Menu Options**:
   - Option 1: Set the number of operations (Admin).
   - Option 2: Play (Participant).
   - Option 3: View results (Observer/Admin).
   - Option 4: Show final output and exit.

## **Project Structure**

The project is organized into the following files:

  - **main.rs** :
Contains the main program logic, including the menu options and interaction between roles.
  - **lib.rs** :
Defines auxiliary structures like **Context** and **WhitelistAuthorizer**.
  - **questions.rs** :
CContains the questions and their logic. You can add or modify questions here.
  - **Cargo.toml** :
Lists the project dependencies, including those from the JAM SDK.

## **Customization**
### **Adding Questions**

PYou can add new questions by editing the **questions.rs**. For example:

```
Question {
    text: "¿What is the capital of France?".to_string(),
    question_type: "multiple_choice".to_string(),
    options: Some(vec![
        "Paris".to_string(),
        "Londons".to_string(),
        "Berlin".to_string(),
    ]),
    answer: Some("Paris".to_string()),
},
```
### **Modifying Roles and Addresses**

EEdit the **main.rs** file to change predefined addresses or add new roles.

## **Contributions**

Contributions are welcome! If you want to improve this project, follow these steps:

  1. Fork the repository.
  2. Create a branch for your contribution (**git checkout -b feature/new-feature**).
  3. Make your changes and submit a pull request.

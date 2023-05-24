# User login API. Written using the AXUM Rust web framework and ScyllaDB database.
 Project provides endpoints for user registration and login functionality. 
 ## Getting Started
 ### Prequisities
  Before you start, ensure that Rust is installed on your system. If you are working on linux, you can install rust using this command:  
  `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh `
 ### Installation
  1. Clone the repository:  
    `git clone https://github.com/dzlk17/login_api.git`  
  2. Navigate to the project directory:  
    `cd user_login_api`  
  3. Build the project:  
    `cargo build`   
  4. Create .env file with your data, for example:  
    `SCYLLA_URI="127.0.0.1:9042"
     DB_NAME="cassandra"
     DB_PASSWORD="cassandra"
     SECRET_KEY="mysecretkey"`
  5. Run docker with database:  
    `docker compose up -d`  
  6. Connect with database:  
    `docker exec -it some-scylla cqlsh --username <name> --password <password>`
  7. Create keyspace and table (code from database_initialization.cql)  
  8. Run the application:  
    `cargo run`
    
 ## Environment setup
   Project created in github codespace in Visual Studio Code with extensions: Docker and Thunder Client.
  
  ![image](https://github.com/dzlk17/login_api/assets/105115971/7c53bdf2-8374-4336-a218-df5947ae7114)

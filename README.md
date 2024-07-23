# ChatGPT Rust API Client

This Rust application demonstrates how to interact with the OpenAI ChatGPT API using the `reqwest` library for making HTTP requests, `serde` for serialization and deserialization, and `tokio` for asynchronous programming. The code allows you to send messages to the OpenAI API and receive a response from the ChatGPT model.

## Features

- Asynchronous API requests using `reqwest` and `tokio`.
- Structured request and response handling with `serde`.
- Easy configuration using environment variables with `dotenv`.
- Input validation for parameters like `temperature`.

## Prerequisites

Before you can use this code, ensure you have the following installed:

- Rust and Cargo: [Install Rust](https://www.rust-lang.org/tools/install)
- OpenAI API key: [Get API Key](https://beta.openai.com/signup/)
- A `.env` file containing your OpenAI API key:
  ```
  OPENAI_API_KEY=your_openai_api_key_here
  ```

## Usage

1. **Clone the Repository**
   ```sh
   git clone https://github.com/Soroushsrd/nexus.git
   cd nexus
   ```

2. **Set Up Environment Variables**
   Create a `.env` file in the root directory of your project and add your OpenAI API key:
   ```sh
   OPENAI_API_KEY=your_openai_api_key_here
   ```

3. **Add Dependencies**
   Ensure your `Cargo.toml` includes the following dependencies:
   ```toml
   [dependencies]
   reqwest = { version = "0.11", features = ["json"] }
   serde = { version = "1.0", features = ["derive"] }
   tokio = { version = "1", features = ["full"] }
   dotenv = "0.15.0"
   ```

4. **Run the Application**
   To run the application, use the following command:
   ```sh
   cargo run
   ```


## Contributing

Feel free to fork this repository, create a new branch, and submit a pull request with your changes. We welcome contributions of all kinds.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

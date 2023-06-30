1. Exported Variables: 
   - `loggedInUser`: Stores the user details after successful login.
   - `currentPlatform`: Stores the platform (Android or iOS) the app is running on.
   - `currentMessage`: Stores the current message being composed or viewed.

2. Data Schemas:
   - `User`: Schema for user details.
   - `Message`: Schema for message details.

3. ID Names of DOM Elements:
   - `loginForm`: Form for logging in.
   - `messageForm`: Form for composing messages.
   - `platformSelect`: Dropdown for selecting the platform to compile for.

4. Message Names:
   - `loginSuccess`: Emitted when a user logs in successfully.
   - `messageSent`: Emitted when a message is sent.
   - `compileSuccess`: Emitted when the app is compiled successfully.

5. Function Names:
   - `login()`: Handles user login.
   - `sendMessage()`: Handles sending messages.
   - `compileApp()`: Handles app compilation.

6. Shared Libraries:
   - `tauri`: Used for compiling the app.
   - `picocss`: Used for styling the app.

7. Shared Configurations:
   - `config.toml`: Contains configuration details for the app.
   - `Cargo.toml`: Contains package and dependency information for the Rust project.

8. Shared Assets:
   - `logo.png`: Logo of the app.
   - `favicon.ico`: Favicon of the app.

9. Shared Test Files:
   - `login.rs`: Tests for login functionality.
   - `messaging.rs`: Tests for messaging functionality.
   - `compile.rs`: Tests for compile functionality.
# Procedural Macros in Rust

Procedural macros in Rust allow code manipulation similar to functions, taking code as input and producing code as output. There are three types: custom derived, attribute-like, and function-like macros. Each type has specific syntax and usage, enabling developers to automate code generation and enhance functionality in Rust applications.

## Highlights

### 00:38 - Introduction to Procedural Macros
Procedural macros in Rust function like regular functions by accepting code as input and producing code as output. They differ from declarative macros that replace code based on patterns.

- **Three Types of Procedural Macros**:
    - Custom derived macros
    - Attribute-like macros
    - Function-like macros
  
- **Tokens**:
    - Tokens in Rust represent the smallest elements of a program, including keywords and identifiers, forming the basis for the input and output of procedural macros.

- **Custom Crate**:
    - Procedural macros require a custom crate type and must be defined within their own crate, which adds complexity to their implementation and usage.

### 02:19 - Implementing Procedural Macros
Implementing macros in Rust requires creating a separate library crate. This process involves defining traits and procedural macros, which enhance code functionality and maintain structure.

- **Bringing Macros into Scope**:
    - Bringing macros into scope is essential for defining traits and ensuring their functionality. This step allows the use of macros within specific structures, like pancakes.

- **Creating Library Crates**:
    - Creating library crates is a standard practice in Rust for organizing code. It helps in structuring macros and their corresponding implementations effectively.

- **Crate Separation**:
    - Procedural macros must be defined in their own crate, adhering to Rust's conventions. This separation ensures that macros and their implementations remain manageable and reusable.

### 04:38 - Creating a Procedural Macro
Creating a procedural macro in Rust involves specific steps, including setting up dependencies and defining the macro's structure. This process enhances code readability and reduces duplication through clear separation of concerns.

- **The `proc-macro` Attribute**:
    - The importance of the `proc-macro` attribute in Rust's macro system signifies that we're dealing with a special type of crate. This attribute is crucial for proper functionality of the macro.

    ```rust
    // In the procedural macro crate
    use proc_macro::TokenStream;

    #[proc_macro]
    pub fn my_macro(input: TokenStream) -> TokenStream {
        input
    }
    ```

- **Using the `syn` Crate**:
    - Using the `syn` crate allows developers to convert raw Rust code into a syntax tree, enabling easier manipulation and understanding of the code structure.

    ```toml
    // Add dependencies to Cargo.toml
    [dependencies]
    syn = "1.0"
    quote = "1.0"
    ```

    ```rust
    use syn::{parse_macro_input, ItemFn};

    #[proc_macro]
    pub fn print_fn(input: TokenStream) -> TokenStream {
        let input = parse_macro_input!(input as ItemFn);
        // Modify the function as needed
        input.to_token_stream()
    }
    ```

- **Parsing and Transforming Operations**:
    - The separation of parsing and transforming operations in macro creation enhances code organization and maintainability. This approach ensures that common parsing logic can be reused across different macros.

### 06:55 - Macro Implementation Process
Implementing a Rust macro involves parsing input into an abstract syntax tree and utilizing templating capabilities to generate code. This process allows for customization and streamlined code generation during compile time.

- **The `unwrap` Function**:
    - The function `unwrap` is crucial as it ensures the program panics on parsing failure, making it suitable for compile-time operations. This avoids unnecessary runtime checks.

    ```rust
    let tokens = syn::parse_str::<ItemFn>("fn hello() {}").unwrap();
    ```

- **Using the `quote` Macro**:
    - The `quote` macro is used to generate Rust code dynamically by replacing variables with actual type names, enhancing code flexibility and maintainability.

    ```rust
    use quote::quote;

    let generated_code = quote! {
        fn generated_function() {
            println!("Hello from generated code!");
        }
    };
    ```

- **Testing Macros**:
    - Testing the macro involves adding crates as dependencies and ensuring compatibility. Successful compilation confirms that the macro behaves as intended and produces the expected output.

    ```toml
    // Add the procedural macros crate as a dependency in the main crate
    [dependencies]
    my_macro = { path = "../my_macro" }
    ```

    ```rust
    // In the main crate
    use my_macro::print_fn;

    print_fn! {
        fn hello() {
            println!("Hello, world!");
        }
    }
    ```

### 09:14 - Attribute-like Macros
Attribute-like macros in Rust are versatile tools that can generate code for various types, including functions. They allow developers to create custom attributes that enhance functionality and streamline coding.

- **Custom Derived Macros**:
    - Custom derived macros are limited to structs and enums, while attribute-like macros can be applied to functions and other types, expanding their usability.

- **Example: Route Attribute**:
    - The example of creating a 'route' attribute illustrates how attribute-like macros can map HTTP requests to specific functions, enhancing web framework capabilities.

    ```rust
    #[route("/home")]
    fn home() {
        println!("Home page");
    }
    ```

- **Function-like Macros**:
    - Function-like macros in Rust provide flexibility by accepting variable arguments and operating directly on Rust code, allowing for dynamic code generation.

    ```rust
    macro_rules! generate_fn {
        ($name:ident) => {
            fn $name() {
                println!("Generated function: {}", stringify!($name));
            }
        };
    }

    generate_fn!(hello);

    fn main() {
        hello(); // Calls the generated function
    }
    ```


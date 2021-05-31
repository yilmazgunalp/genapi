## This is the API that creates an API using the **genapi_macro** crate contained in *meta_rust* library 

**It creates an API in these steps:**

1. Create an **output_api** directory with *cargo.toml* which includes **genapi_macro** crate.(This is currently done manually once)

2. Write macros to the *main.rs* file of the *output_api*.(This is currently done using rams mustache template)

3. Compile the *output_api* to *output_binary* folder.

4. Send the output_binary to client   
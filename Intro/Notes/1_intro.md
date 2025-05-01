# Print `Hello world` 

### Using `cli`

- create `hello.rs` file 

```rs
fn main(){
    println!("Hello, world ");
}
```

- on cli 

```bash 
 rustc hello.rs
```
creates an executable  `hello.exe`

- To print hello world on terminal 

```bash
./hello
```


### Using `Cargo Package Manager`    

- Create project with `cargo`

```bash 
cargo new helloProject
```

##### Files created 
- .toml  - project settings - adding dependencies 
- src - main project code .

##### To run project 

```sh 
cd helloProject 
cargo run 
```






### Enums

- A versatile tool used to represent a type that can take one of several possible variants 
- For holding related types 

```rs 

enum IpAddrKind{
    V4,
    V6
};

// CREATE instance of enum 

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;


// Can be used as parameters in funtions 

fn route (ip_kind: IpAddrKind){

}


enum IpAddrKind{
    V4,
    V6
};

// Creating a struct for storing the ip address . 

struct IpAddr{
    kind : IpAddrKind , // can either be v4/v6
    address : String
};

let home = IpAddr{
    kind:IpAddrKind::V4 , 
    address: String::from("127.0.0.1"),
};

let loop_back = IpAddr{
    kind:IpAddrKind::V6 , 
    address: String::from("::1"),
}







// we can define the data type ... like having instances ... without using   structs ..
enum IpAddrKindNamed{
    V4(String),
    V6(String)
};

let home = IpAddrKindNamed::V4(String::from("qa"))
let loopback = IpAddrKindNamed::V6(String::from("qa"))










// Enhanced Enums 

enum IpAddrEnhanced{
    V4(u8,u8,u8,u8),
    V6(String)
};


let home = IpAddrEnhanced::V4(String::from(1,2,2,3))
let loopback = IpAddrEnhanced::V6(String::from("qa"))

```
# create new migration for table .. 

```sh
# create new table post
sea-orm-cli migrate generate create_post_tables

# run migrations 
sea-orm-cli migrate up 

# generate entities 
sea-orm-cli generate entity -o entity/src
# the relationships are auto defined when you generate the entities .. 
```


# To join two tables .. .

- Use a `Custom relation` or a `Prebuilt relation ` 



## Take note of this 


```rs

impl From<(entity::post::Model, Option<entity::user::Model>)> for PostModel {
 fn from(value : (entity::post::Model, Option<entity::user::Model>)) -> Self {
    let u = value.1.unwrap();
    return PostModel{
        uuid : value.0.uuid,
        text : value.0.text,
        created_at : value.0.created_at,
        title : value.0.title,
        id : value.0.id,
        image : value.0.image,
        user_id : value.0.user_id,  
        user : Some(UserMicroModel { name: u.name, uuid:u.uuid })  // user info
    };
 }
}

/// apply .into where you wish to apply it .
.into()

```



## In this project . we did `Prebuilt relation ` 

- Checkout `Custom relation joins` in the sea orm docs ... 



## Serde json 

- `Value` . A flexible type .. 



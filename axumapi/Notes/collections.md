

```rs

 let all_user : Vec<UserModel> =  entity::user::Entity::find().all(&db).await.unwrap().into_iter().map(|item|UserModel::new(item)).collect();


```



### Into_iter , iter , iter_mut 

```rs 


// T
.into_iter()

// &T
.iter()

// &mut T
.iter_mut()


```


### .map 


### .collect

- This returns the data how you want .. you'll have to put type annotation at the variable initalization when using .collect() 

```rs

let a: vec<u32> = data.collect()

```


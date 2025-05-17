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
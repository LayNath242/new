use diesel::PgConnection;
use chrono::NaiveDateTime;
use crate::models::user::User;
use crate::schema::{comments, users};

#[derive(Queryable, Serialize, Deserialize, Debug, PartialEq)]
pub struct Comment {
    pub id: i32,
    pub description: String,
    pub user_id: i32,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize, Serialize, AsChangeset, Debug, Clone, Associations ,PartialEq)]
#[belongs_to(User)]
#[table_name="comments"]
pub struct NewComment {
    pub description: String,
    pub user_id: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
}


#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct UserComment {
    pub id: i32,
    pub description: String,
    pub name: String,
    pub created_at: Option<NaiveDateTime>,
}

type Columns = (
    comments::id,
    comments::description,
    users::fullname,
    comments::created_at,
    
);

const COLUMNS: Columns = (
    comments::id,
    comments::description,
    users::fullname,
    comments::created_at,  
);

#[derive(Serialize, Deserialize)]
pub struct Commentlist(pub Vec<UserComment>);



impl Commentlist {
    pub fn list(connection: &PgConnection) -> Self {
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        // use crate::schema::comments::dsl::*;

        let result = 
            users::table
                .inner_join(comments::table)
                .select(COLUMNS)
                .load::<UserComment>(connection)
                
                .expect("Error loading comment");

        Commentlist(result)
    }
}


impl NewComment {
     pub fn create(&self, connection: &PgConnection) ->
        Result<Comment, diesel::result::Error> {
            use diesel::RunQueryDsl;

                diesel::insert_into(comments::table)
                    .values(self)
                    .get_result(connection)
        }
}

impl Comment {
    pub fn find(id: &i32, connection: &PgConnection) -> Result<Comment, diesel::result::Error>{
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;

        comments::table.find(id).first(connection)
    }

    pub fn destroy(id: &i32, connection: &PgConnection) -> Result<(), diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::schema::comments::dsl;

        diesel::delete(dsl::comments
        .find(id))
        .execute(connection)?;
        Ok(())
    }


    pub fn update(id: &i32,
                  new_comment : &NewComment, 
                  connection: &PgConnection) 
                  -> Result<(), diesel::result::Error>{
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use crate::schema::comments::dsl;

        diesel::update(dsl::comments.find(id))
            .set(new_comment)
            .execute(connection)?;
            Ok(())
    }   

}
#![allow(dead_code)]

use juniper::*;

type Schema = juniper::RootNode<'static, Query, EmptyMutation<Context>>;

pub struct Context {
    user: User,
}

impl juniper::Context for Context {}

pub struct Query;

graphql_object!(Query: Context |&self| {
    field user(&executor) -> FieldResult<User> {
        executor.look_ahead(); // <-- OK

        let ctx = executor.context();
        let user = ctx.user.clone();
        Ok(user)
    }
});

#[derive(Clone, GraphQLObject)]
pub struct Country {
    id: i32,
}

#[derive(Clone)]
pub struct User {
    country: Country,
}

graphql_object!(User: Context |&self| {
    field country(&executor) -> FieldResult<&Country> {
        executor.look_ahead(); // <-- PANIC

        Ok(&self.country)
    }
});

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn with_fragment() {
        let user = User {
            country: Country { id: 1 },
        };

        let context = Context { user };

        let (res, _errors) = juniper::execute(
            r#"
                query Foo {
                  user {
                    ...userFields
                  }
                }

                fragment userFields on User {
                  country { id }
                }
            "#,
            None,
            &Schema::new(Query, EmptyMutation::new()),
            &Variables::new(),
            &context,
        )
        .unwrap();

        let res = serde_json::to_value(res).expect("convert to `serde_json::Value`");

        assert_eq!(res["user"]["country"]["id"].as_i64().unwrap(), 1);
    }

    #[test]
    fn without_fragment() {
        let user = User {
            country: Country { id: 1 },
        };

        let context = Context { user };

        let (res, _errors) = juniper::execute(
            r#"
                query Foo {
                  user {
                    country { id }
                  }
                }
            "#,
            None,
            &Schema::new(Query, EmptyMutation::new()),
            &Variables::new(),
            &context,
        )
        .unwrap();

        let res = serde_json::to_value(res).expect("convert to `serde_json::Value`");

        assert_eq!(res["user"]["country"]["id"].as_i64().unwrap(), 1);
    }
}

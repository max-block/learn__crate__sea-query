use sea_query::{Iden, Query, Expr, PostgresQueryBuilder};

#[derive(Iden)]
enum Post {
    Table,
    Id,
    Title,
    CreatedAt,
}

fn main() {
    let query = Query::update()
        .table(Post::Table)
        .value(Post::Title, "qwe".into()).and_where(Expr::col(Post::Id).eq(19)).to_owned();

    let sql = query.to_string(PostgresQueryBuilder);
    dbg!(sql);
    // "UPDATE \"post\" SET \"title\" = 'qwe' WHERE \"id\" = 19"
    
}

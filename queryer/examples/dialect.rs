use sqlparser::{
    dialect::GenericDialect,
    parser::Parser,
};

fn main() {
    tracing_subscriber::fmt::init();

    let sql = "SELECT a, b, 123, myfunc(b) \
           FROM table_1 \
           WHERE a > b AND b < 100 \
           ORDER BY a DESC, b";

    let ast = Parser::parse_sql(&GenericDialect::default(), sql);
    println!("{:?}", ast);
}




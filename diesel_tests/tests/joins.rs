use super::schema::*;
use diesel::*;

#[test]
fn belongs_to() {
    let connection = connection();
    setup_users_table(&connection);
    setup_posts_table(&connection);

    connection.execute("INSERT INTO users (name) VALUES ('Sean'), ('Tess')")
        .unwrap();
    connection.execute("INSERT INTO posts (user_id, title, body) VALUES
        (1, 'Hello', 'Content'),
        (2, 'World', DEFAULT)
    ").unwrap();

    let sean = User::new(1, "Sean");
    let tess = User::new(2, "Tess");
    let seans_post = Post::new(1, 1, "Hello", Some("Content"));
    let tess_post = Post::new(2, 2, "World", None);

    let expected_data = vec![(seans_post, sean), (tess_post, tess)];
    let source = posts::table.inner_join(users::table);
    let actual_data: Vec<_> = source.load(&connection).unwrap().collect();

    assert_eq!(expected_data, actual_data);
}

#[test]
fn select_single_from_join() {
    let connection = connection();
    setup_users_table(&connection);
    setup_posts_table(&connection);

    connection.execute("INSERT INTO users (name) VALUES ('Sean'), ('Tess')")
        .unwrap();
    connection.execute("INSERT INTO posts (user_id, title) VALUES
        (1, 'Hello'),
        (2, 'World')
    ").unwrap();

    let source = posts::table.inner_join(users::table);
    let select_name = source.select(users::name);
    let select_title = source.select(posts::title);

    let expected_names = vec!["Sean".to_string(), "Tess".to_string()];
    let actual_names: Vec<String> = select_name.load(&connection).unwrap().collect();

    assert_eq!(expected_names, actual_names);

    let expected_titles = vec!["Hello".to_string(), "World".to_string()];
    let actual_titles: Vec<String> = select_title.load(&connection).unwrap().collect();

    assert_eq!(expected_titles, actual_titles);
}

#[test]
fn select_multiple_from_join() {
    let connection = connection();
    setup_users_table(&connection);
    setup_posts_table(&connection);

    connection.execute("INSERT INTO users (name) VALUES ('Sean'), ('Tess')")
        .unwrap();
    connection.execute("INSERT INTO posts (user_id, title) VALUES
        (1, 'Hello'),
        (2, 'World')
    ").unwrap();

    let source = posts::table.inner_join(users::table)
        .select((users::name, posts::title));

    let expected_data = vec![
        ("Sean".to_string(), "Hello".to_string()),
        ("Tess".to_string(), "World".to_string()),
    ];
    let actual_data: Vec<_> = source.load(&connection).unwrap().collect();

    assert_eq!(expected_data, actual_data);
}

#[test]
fn select_only_one_side_of_join() {
    let connection = connection();
    setup_users_table(&connection);
    setup_posts_table(&connection);

    connection.execute("INSERT INTO users (name) VALUES ('Sean'), ('Tess')")
        .unwrap();
    connection.execute("INSERT INTO posts (user_id, title) VALUES (2, 'Hello')")
        .unwrap();

    let source = users::table.inner_join(posts::table).select(users::all_columns);

    let expected_data = vec![User::new(2, "Tess")];
    let actual_data: Vec<_> = source.load(&connection).unwrap().collect();

    assert_eq!(expected_data, actual_data);
}

#[test]
fn left_outer_joins() {
    let connection = connection();
    setup_users_table(&connection);
    setup_posts_table(&connection);

    connection.execute("INSERT INTO users (name) VALUES ('Sean'), ('Tess')")
        .unwrap();
    connection.execute("INSERT INTO posts (user_id, title) VALUES
        (1, 'Hello'),
        (1, 'World')
    ").unwrap();

    let sean = User::new(1, "Sean");
    let tess = User::new(2, "Tess");
    let seans_post = Post::new(1, 1, "Hello", None);
    let seans_second_post = Post::new(2, 1, "World", None);

    let expected_data = vec![
        (sean.clone(), Some(seans_post)),
        (sean, Some(seans_second_post)),
        (tess, None)
    ];
    let source = users::table.left_outer_join(posts::table);
    let actual_data: Vec<_> = source.load(&connection).unwrap().collect();

    assert_eq!(expected_data, actual_data);
}

#[test]
fn columns_on_right_side_of_left_outer_joins_are_nullable() {
    let connection = connection();
    setup_users_table(&connection);
    setup_posts_table(&connection);

    connection.execute("INSERT INTO users (name) VALUES ('Sean'), ('Tess')")
        .unwrap();
    connection.execute("INSERT INTO posts (user_id, title) VALUES
        (1, 'Hello'),
        (1, 'World')
    ").unwrap();

    let expected_data = vec![
        ("Sean".to_string(), Some("Hello".to_string())),
        ("Sean".to_string(), Some("World".to_string())),
        ("Tess".to_string(), None),
    ];
    let source = users::table.left_outer_join(posts::table).select((users::name, posts::title));
    let actual_data: Vec<_> = source.load(&connection).unwrap().collect();

    assert_eq!(expected_data, actual_data);
}

#[test]
fn select_multiple_from_right_side_returns_optional_tuple() {
    let connection = connection();
    setup_users_table(&connection);
    setup_posts_table(&connection);

    connection.execute("INSERT INTO users (name) VALUES ('Sean'), ('Tess')")
        .unwrap();
    connection.execute("INSERT INTO posts (user_id, title, body) VALUES
        (1, 'Hello', 'Content'),
        (1, 'World', DEFAULT)
    ").unwrap();

    let expected_data = vec![
        Some(("Hello".to_string(), Some("Content".to_string()))),
        Some(("World".to_string(), None)),
        None,
    ];

    let source = users::table.left_outer_join(posts::table).select((posts::title, posts::body));
    let actual_data: Vec<_> = source.load(&connection).unwrap().collect();

    assert_eq!(expected_data, actual_data);
}

#[test]
fn select_complex_from_left_join() {
    let connection = connection();
    setup_users_table(&connection);
    setup_posts_table(&connection);

    connection.execute("INSERT INTO users (name) VALUES ('Sean'), ('Tess')")
        .unwrap();
    connection.execute("INSERT INTO posts (user_id, title, body) VALUES
        (1, 'Hello', 'Content'),
        (1, 'World', DEFAULT)
    ").unwrap();

    let sean = User::new(1, "Sean");
    let tess = User::new(2, "Tess");
    let expected_data = vec![
        (sean.clone(), Some(("Hello".to_string(), Some("Content".to_string())))),
        (sean, Some(("World".to_string(), None))),
        (tess, None),
    ];

    let source = users::table.left_outer_join(posts::table).select((users::all_columns, (posts::title, posts::body)));
    let actual_data: Vec<_> = source.load(&connection).unwrap().collect();

    assert_eq!(expected_data, actual_data);
}

#[test]
fn select_right_side_with_nullable_column_first() {
    let connection = connection();
    setup_users_table(&connection);
    setup_posts_table(&connection);

    connection.execute("INSERT INTO users (name) VALUES ('Sean'), ('Tess')")
        .unwrap();
    connection.execute("INSERT INTO posts (user_id, title, body) VALUES
        (1, 'Hello', 'Content'),
        (1, 'World', DEFAULT)
    ").unwrap();

    let sean = User::new(1, "Sean");
    let tess = User::new(2, "Tess");
    let expected_data = vec![
        (sean.clone(), Some((Some("Content".to_string()), "Hello".to_string()))),
        (sean, Some((None, "World".to_string()))),
        (tess, None),
    ];

    let source = users::table.left_outer_join(posts::table).select((users::all_columns, (posts::body, posts::title)));
    let actual_data: Vec<_> = source.load(&connection).unwrap().collect();

    assert_eq!(expected_data, actual_data);
}

#[test]
fn select_then_join() {
    use schema::users::dsl::*;
    let connection = connection_with_sean_and_tess_in_users_table();
    setup_posts_table(&connection);

    connection.execute("INSERT INTO posts (user_id, title) VALUES (1, 'Hello')")
        .unwrap();
    let expected_data = vec![1];
    let data: Vec<_> = users.select(id).inner_join(posts::table)
        .load(&connection).unwrap().collect();

    assert_eq!(expected_data, data);

    let expected_data = vec![1, 2];
    let data: Vec<_> = users.select(id).left_outer_join(posts::table)
        .load(&connection).unwrap().collect();

    assert_eq!(expected_data, data);
}

#[test]
fn join_through_other() {
    use schema::users::dsl::*;
    let connection = connection_with_sean_and_tess_in_users_table();
    setup_posts_table(&connection);
    setup_comments_table(&connection);

    connection.insert_returning_count(&users, &NewUser::new("Jim", None))
        .unwrap();
    connection.insert_returning_count(&posts::table, &vec![
        NewPost::new(1, "Hello", None), NewPost::new(2, "World", None),
        NewPost::new(1, "Hello again!", None),
    ]).unwrap();
    let comments: Vec<Comment> = connection.insert(&comments::table, &vec![
        NewComment(1, "OMG"), NewComment(2, "WTF"),
        NewComment(3, "Best post ever!!!")]).unwrap().collect();

    let data: Vec<_> = users.inner_join(comments::table).load(&connection)
        .unwrap().collect();

    let sean = User::new(1, "Sean");
    let tess = User::new(2, "Tess");
    let expected_data = vec![
        (sean.clone(), comments[0].clone()),
        (tess, comments[1].clone()),
        (sean, comments[2].clone()),
    ];
    assert_eq!(expected_data, data);
}

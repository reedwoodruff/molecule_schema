#[cfg(test)]
mod tests {

    use generate_schema_macro::generate_constraint_schema;
    #[test]
    fn test_macro() {
        generate_constraint_schema!();
        // println!("{:?}", constraint_schema);
        // panic!();
        let test = Schema::HasColoredObject(HasColoredObject::initiate_build().build().unwrap());
        let test2 = Schema::Sock(
            Sock::initiate_build()
                .set_color("blue".to_string())
                .build()
                .unwrap(),
        );
        let test3 = Schema::Person(
            Person::initiate_build()
                .set_name("blubber".to_string())
                .build()
                .unwrap(),
        );
        println!("{:?}", test);
        println!("{:?}", test2);
        println!("{:?}", test3);
        println!("{:?}", test2.get_constraint_schema_id());
        panic!();
    }
}

extern crate cassandra;

use cassandra::*;

fn main() {

    let mut cluster = CassCluster::new();
    cluster.set_contact_points("127.0.0.1").unwrap();

    let session = CassSession::new().connect(&cluster).wait().unwrap();
    let schema = session.get_schema();
    session.execute(
        "CREATE KEYSPACE examples WITH replication = \
        { 'class': 'SimpleStrategy', 'replication_factor': '3' }",
        0
    );

    session.execute(
        "CREATE TYPE examples.phone_numbers (phone1 int, phone2 int)",
        0
    );

    session.execute(
        "CREATE TYPE examples.address \
        (street text, city text, zip int, phone set<frozen<phone_numbers>>)"
        ,0
    );

    session.execute(
        "CREATE TABLE examples.udt (id timeuuid, address frozen<address>, PRIMARY KEY(id))",
        0
    );

    insert_into_udt(&session, schema).unwrap();
    select_from_udt(&session).unwrap();
    session.close().wait().unwrap();
}

fn select_from_udt(session: &CassSession) -> Result<(), CassError> {
    let query = "SELECT * FROM examples.udt";
    let statement = CassStatement::new(query, 0);
    let mut future = session.execute_statement(&statement);
    match future.wait() {
        Err(err) => panic!("Error: {:?}", err),
        Ok(result) => {
            for row in result.iter() {
                let id_value = row.get_column_by_name("id");
                let address_value = row.get_column_by_name("address");
                let fields_iter = try!(address_value.use_type_iter());
                let id_str = try!(id_value.get_uuid()).to_string();
                println!("id {}", id_str);
                for field in fields_iter {
                    println!("{}", field.0);
                    match field.1.get_type() {
                        CassValueType::VARCHAR => println!("{}", try!(field.1.get_string())),
                        CassValueType::INT => println!("{}", try!(field.1.get_int32())),
                        CassValueType::SET =>
                            for phone_numbers in try!(field.1.as_set_iterator()) {
                            for phone_number in try!(phone_numbers.as_user_type_iterator()) {
                                let phone_number_value = phone_number.1;
                                println!("{}", phone_number_value);
                            }
                        },
                        other => panic!("Unsupported type: {:?}", other),
                    }
                }
            }
            Ok(())

        }
    }
}

fn insert_into_udt(session: &CassSession, schema: CassSchema) -> Result<(), CassError> {
    let query = "INSERT INTO examples.udt (id, address) VALUES (?, ?)";
    let mut statement = CassStatement::new(query, 2);
    let uuid_gen = CassUuidGen::new();
    let udt_address = schema.get_udt("examples", "address");
    let udt_phone = CassSchema::get_udt(&schema, "examples", "phone_numbers");
    let id = uuid_gen.get_time();
    let id_str = id.to_string();
    let mut address = CassUserType::new(udt_address);
    let mut phone = CassSet::new(2);
    let mut phone_numbers = CassUserType::new(udt_phone);
    phone_numbers.set_int32_by_name("phone1", 0 + 1).unwrap();
    phone_numbers.set_int32_by_name("phone2", 0 + 2).unwrap();
    phone.append_user_type(phone_numbers).unwrap();
    address.set_string_by_name("street", &id_str).unwrap();
    address.set_int32_by_name("zip", id.0.time_and_version as i32).unwrap();
    address.set_collection_by_name("phone", phone).unwrap();

    statement.bind_uuid(0, id).unwrap();
    statement.bind_user_type(1, address).unwrap();
    let mut future = session.execute_statement(&statement);
    match future.wait() {
        Ok(_) => Ok(()),
        Err(err) => panic!("Error: {:?}", err),
    }
}

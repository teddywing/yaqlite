use rusqlite;
use yaml_rust;

fn main() {
    println!("Hello, world!");

    let text_data = std::fs::read_to_string("test.yml").unwrap();

    {
        use yaml_rust::YamlLoader;

        let yaml_data = YamlLoader::load_from_str(&text_data).unwrap();

        dbg!(yaml_data);
    }
}

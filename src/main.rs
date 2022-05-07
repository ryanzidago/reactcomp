use std::env;
use std::fs::File;
use std::fs;
use std::io::Write;
use std::path::Path;
const USAGE: &str = "
Usage: 
    reactcomp <component name> <optional target directory>

Example:
    reactcomp MyComponent components
";

fn main() {
    let default_dir = ".".to_string();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("{}", USAGE);
        return;
    }

    let component_name = args.get(1).unwrap();
    let target_dir = args.get(2).unwrap_or(&default_dir);

    if !Path::new(target_dir).is_dir() {
        fs::create_dir(target_dir).expect("Failed to create directory {target_dir}");
    }

    let component: String = create_component_as_string(component_name);
    let filename = format!("{target_dir}/{component_name}.js");
    let mut file = File::create(&filename).expect("Failed to create file {filename}.js");

    write!(file, "{}", component).expect("Failed to write to file {filename}.js");

    println!("Created {filename}");
}

fn create_component_as_string(component_name: &str) -> String {
    "
import {View, StyleSheet} from 'react-native';
    
const TemplateComponent = ({}) => {
    return (
        <View style={styles.container}>
        </View>
    )
}
    
const styles = StyleSheet.create({
    container: {}
})
    
export default TemplateComponent;
        ".to_string()
        .trim_start()
        .replace("TemplateComponent", component_name)
}

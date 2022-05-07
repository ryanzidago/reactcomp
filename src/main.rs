use std::env;
use std::fs::File;
use std::fs;
use std::io::Write;
use std::path::Path;

const USAGE: &str = "
Usage: 
    reactcomp <component name> <optional target directory> <optional extension>

Examples:
    reactcomp MyComponent
    reactcomp MyComponent components/ .js
    reactcomp MyComponent components/ .android.js
";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("{}", USAGE);
        return;
    }

    let component_name = args.get(1).unwrap();
    
    let default_dir = ".".to_string();
    let target_dir = args.get(2).unwrap_or(&default_dir);

    let default_extension = ".js".to_string();
    let extension = args.get(3).unwrap_or(&default_extension);

    if !Path::new(target_dir).is_dir() {
        fs::create_dir(target_dir).expect("Failed to create directory {target_dir}");
    }

    let component: String = create_component_as_string(component_name);
    let filename = format!("{target_dir}/{component_name}{extension}");
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

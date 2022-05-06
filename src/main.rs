use std::env;
use std::fs::File;
use std::io::Write;

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

    let component: String = create_component_as_string(component_name);
    let filename = format!("{target_dir}/{component_name}.js");
    let mut file = File::create(&filename).expect("Failed to create file {filename}.js");

    write!(file, "{}", component).expect("Failed to write to file {filename}.js");

    println!("Created {filename}");
}

fn create_component_as_string(component_name: &str) -> String {
    "
import {View, Stylesheet} from 'react-native';
    
const TemplateComponent = ({}) => {
    return (
        <View style={styles.container}>
        </View>
    )
}
    
const styles = Stylesheet.create({
    container: {}
})
    
export default TemplateComponent;
        ".to_string()
        .trim_start()
        .replace("TemplateComponent", component_name)
}

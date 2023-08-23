use clap::Parser;
use std::{fs, env, io};


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Type of Godot project
    #[arg(short, long)]
    project_type: String,

    /// Name
    #[arg(short, long)]
    name: String
}

fn main() {
    let args = Args::parse();
    let mut author = String::new();
    let mut description = String::new();

    println!("Enter author:");
    let _ = io::stdin().read_line(&mut author);

    println!("Enter description:");
    let _ = io::stdin().read_line(&mut description);
    
    //TODO validate args
    let project_name = &args.name;
    let project_directory_name = project_name.to_lowercase();
    let project_child_directories = ["src", "test"];

    let tool_file = Template {
        from_name: String::from("tool_file"),
        to_name: project_name.to_lowercase(),
        extension: String::from("gd"),
        data:  vec![]
    };

    let plugin_file = Template {
        from_name: String::from("plugin"),
        to_name: String::from("plugin"),
        extension: String::from("cfg"),
        data:  vec![
            [String::from("TEMPLATE_NAME"), project_name.to_string()],
            [String::from("TEMPLATE_DESCRIPTION"), description],
            [String::from("TEMPLATE_AUTHOR"), author],
            [String::from("TEMPLATE_SCRIPT"), format!("{0}.gd", project_directory_name)]]
    };
    
    let files_to_copy = [tool_file, plugin_file];

    let mut working_directory = env::current_dir().unwrap();
    working_directory.pop();
    working_directory.pop();

    let module_path = format!("{0}/addons/{1}", project_name, project_directory_name);

    for project_child_directory in project_child_directories {
        let path = format!("{0}/{1}", module_path, project_child_directory);
        let created_paths_result = fs::create_dir_all(path);
         
        //TODO get just the error messege
        match created_paths_result {
            Ok(file) => file,
            Err(error) => panic!("Unable to create file: {:?}", error)
        };
    }

    for file_to_copy in files_to_copy {
        let template_file_path = format!("{0}/templates/{1}.{2}", working_directory.display(), file_to_copy.from_name, file_to_copy.extension);
        let write_to_path = format!("{0}/{1}.{2}", module_path, file_to_copy.to_name, file_to_copy.extension);
        let file_contents_result = fs::read_to_string(template_file_path);
        
        let mut file_contents = match file_contents_result {
            Ok(result) => result,
            Err(error) => panic!("Unable to read file: {:?}", error)
        };

        for item in &file_to_copy.data {
            file_contents = file_contents.replace(&item[0], &item[1]);
        }

        let file_written = fs::write(write_to_path, file_contents);

        match file_written {
            Ok(result) => result,
            Err(error) => panic!("Unable to write file: {:?}", error)
        };
    }
}

struct Template {
    from_name: String,
    to_name: String,
    extension: String,
    data: Vec<[String; 2]>
}